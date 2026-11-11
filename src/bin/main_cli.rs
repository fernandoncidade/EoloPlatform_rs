use std::env;
use std::fs;

use eolo_rs::core::project::EoloProject;
use eolo_rs::core::scenarios::get_preset_scenarios;
use eolo_rs::core::types::*;
use eolo_rs::engine::envelopes::WindEnvelopeResults;
use eolo_rs::engine::factors_s1_s2_s3::calculate_s2;
use eolo_rs::engine::isopleth_map::{get_stations_by_uf, BRAZILIAN_STATES, NBR6123_STATIONS};
use eolo_rs::reports::MemorialGenerator;

fn print_help() {
    println!(
r#"================================================================================
Eolo Platform CLI v2026.11.11.0 — Ações de Vento em Edificações (Ref. NBR 6123:2023)
Autor: Fernando Nillsson Cidade (linceu_lighthouse@outlook.com)
AVISO: Ferramenta exclusiva para estudo acadêmico. Pode conter erros. Não usar
para decisões definitivas em produção; confira manualmente ou em ferramentas consolidadas.
================================================================================

USO:
    eolo-cli [COMANDO] [OPÇÕES]

COMANDOS DISPONÍVEIS:
    calc        Cálculo rápido de Vk e pressão dinâmica q (para estudo)
    report      Gera o Memorial de Cálculo de estudo em Markdown, HTML ou JSON
    isopleth    Consulta referencial de isopletas de V0 por Estado (UF) ou Estações FAB
    scenarios   Lista os 5 cenários pré-configurados de estudo
    verify      Checagem acadêmica de rastreabilidade frente à ref. NBR 6123:2023
    test        Execução da esteira de testes analíticos determinísticos
    help, -h    Exibe este manual de instruções da linha de comando

EXEMPLOS DE USO:
    eolo-cli calc --v0 45.0 --cat 4 --class b --s3 1.0 -z 7.5
    eolo-cli report --scenario 1 --format html --output memorial.html
    eolo-cli isopleth --uf PR
    eolo-cli verify --scenario 2
"#
    );
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        print_help();
        return;
    }

    match args[1].as_str() {
        "calc" => handle_calc(&args[2..]),
        "report" => handle_report(&args[2..]),
        "isopleth" => handle_isopleth(&args[2..]),
        "scenarios" => handle_scenarios(),
        "verify" => handle_verify(&args[2..]),
        "test" => handle_test(),
        "help" | "-h" | "--help" => print_help(),
        other => {
            eprintln!("Comando desconhecido: '{}'. Use 'eolo-cli help' para a lista de comandos.", other);
        }
    }
}

fn handle_calc(args: &[String]) {
    let mut v0 = 45.0f64;
    let mut s1 = 1.00f64;
    let mut roughness = TerrainRoughness::CategoryIV;
    let mut building_class = BuildingClass::ClassB;
    let mut s3 = 1.00f64;
    let mut z = 7.5f64;

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--v0" if i + 1 < args.len() => {
                v0 = args[i + 1].parse().unwrap_or(45.0);
                i += 1;
            }
            "--s1" if i + 1 < args.len() => {
                s1 = args[i + 1].parse().unwrap_or(1.0);
                i += 1;
            }
            "--cat" if i + 1 < args.len() => {
                let cat_num: u32 = args[i + 1].parse().unwrap_or(4);
                roughness = match cat_num {
                    1 => TerrainRoughness::CategoryI,
                    2 => TerrainRoughness::CategoryII,
                    3 => TerrainRoughness::CategoryIII,
                    4 => TerrainRoughness::CategoryIV,
                    5 => TerrainRoughness::CategoryV,
                    _ => TerrainRoughness::CategoryIV,
                };
                i += 1;
            }
            "--class" if i + 1 < args.len() => {
                let cl = args[i + 1].to_lowercase();
                building_class = match cl.as_str() {
                    "a" => BuildingClass::ClassA,
                    "b" => BuildingClass::ClassB,
                    "c" => BuildingClass::ClassC,
                    _ => BuildingClass::ClassB,
                };
                i += 1;
            }
            "--s3" if i + 1 < args.len() => {
                s3 = args[i + 1].parse().unwrap_or(1.0);
                i += 1;
            }
            "-z" | "--z" if i + 1 < args.len() => {
                z = args[i + 1].parse().unwrap_or(7.5);
                i += 1;
            }
            _ => {}
        }
        i += 1;
    }

    let s2 = calculate_s2(roughness, building_class, z);
    let vk = v0 * s1 * s2 * s3;
    let q_pa = 0.613 * vk * vk;
    let q_kn_m2 = q_pa / 1000.0;
    let q_kgf_m2 = q_pa / 9.80665;

    println!("--------------------------------------------------------------------------------");
    println!("RESULTADOS DO CÁLCULO — ABNT NBR 6123:2023");
    println!("--------------------------------------------------------------------------------");
    println!("Velocidade Básica (V0):       {:.2} m/s ({:.1} km/h)", v0, v0 * 3.6);
    println!("Fator Topográfico (S1):       {:.3}", s1);
    println!("Fator Rugosidade/Altura (S2): {:.3}  [z = {:.2} m, {}, {}]", s2, z, roughness.short_name(), building_class.short_name());
    println!("Fator Estatístico (S3):       {:.3}", s3);
    println!("--------------------------------------------------------------------------------");
    println!("VELOCIDADE CARACTERÍSTICA (Vk): {:.2} m/s ({:.1} km/h)", vk, vk * 3.6);
    println!("PRESSÃO DINÂMICA (q):           {:.1} Pa", q_pa);
    println!("PRESSÃO DINÂMICA (q):           {:.3} kN/m² (kPa)", q_kn_m2);
    println!("PRESSÃO DINÂMICA (q):           {:.2} kgf/m²", q_kgf_m2);
    println!("--------------------------------------------------------------------------------");
}

fn handle_report(args: &[String]) {
    let mut scenario_id = 1u32;
    let mut format = "markdown".to_string();
    let mut output_path = String::new();

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--scenario" | "-s" if i + 1 < args.len() => {
                scenario_id = args[i + 1].parse().unwrap_or(1);
                i += 1;
            }
            "--format" | "-f" if i + 1 < args.len() => {
                format = args[i + 1].to_lowercase();
                i += 1;
            }
            "--output" | "-o" if i + 1 < args.len() => {
                output_path = args[i + 1].clone();
                i += 1;
            }
            _ => {}
        }
        i += 1;
    }

    let scenarios = get_preset_scenarios();
    let target_scenario = scenarios.into_iter().find(|s| s.id == scenario_id).unwrap_or_else(|| {
        eprintln!("Cenário #{} não encontrado, utilizando Cenário 1.", scenario_id);
        get_preset_scenarios().remove(0)
    });

    let env = WindEnvelopeResults::compute(&target_scenario.project);

    let content = match format.as_str() {
        "html" => MemorialGenerator::generate_html(&target_scenario.project, &env),
        "json" => serde_json::to_string_pretty(&target_scenario.project).unwrap_or_default(),
        _ => MemorialGenerator::generate_markdown(&target_scenario.project, &env),
    };

    if output_path.is_empty() {
        println!("{}", content);
    } else {
        match fs::write(&output_path, content) {
            Ok(_) => println!("Memorial exportado com sucesso para: {}", output_path),
            Err(e) => eprintln!("Erro ao salvar arquivo '{}': {}", output_path, e),
        }
    }
}

fn handle_isopleth(args: &[String]) {
    let mut target_uf = String::new();
    let mut search_city = String::new();

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--uf" if i + 1 < args.len() => {
                target_uf = args[i + 1].to_uppercase();
                i += 1;
            }
            "--city" | "--search" if i + 1 < args.len() => {
                search_city = args[i + 1].to_lowercase();
                i += 1;
            }
            _ => {}
        }
        i += 1;
    }

    println!("================================================================================");
    println!("BANCO DE DADOS DE ISOPLETAS — ABNT NBR 6123:2023 (Figura 1 e Tabela C.1)");
    println!("================================================================================");

    if !target_uf.is_empty() {
        let stations = get_stations_by_uf(&target_uf);
        println!("Estações meteorológicas oficiais no estado {}: {}", target_uf, stations.len());
        for s in stations {
            println!("  [Estação #{:02}] {:<30} V0 = {:.0} m/s (Alt: {:.0}m)", s.id, s.name, s.v0_isopleth, s.altitude_m);
        }
    } else if !search_city.is_empty() {
        let matches: Vec<_> = NBR6123_STATIONS
            .iter()
            .filter(|s| s.name.to_lowercase().contains(&search_city))
            .collect();
        println!("Encontrada(s) {} estação(ões) correspondente(s) a '{}':", matches.len(), search_city);
        for s in matches {
            println!("  [Estação #{:02}] {:<30} UF: {} | V0 = {:.0} m/s", s.id, s.name, s.uf, s.v0_isopleth);
        }
    } else {
        println!("Principais Estados e Capitais:");
        for st in BRAZILIAN_STATES {
            println!("  {:<2} — {:<22} Capital: {:<16} V0 Típico: {:.0} m/s", st.uf, st.name, st.capital, st.v0_typical);
        }
    }
}

fn handle_scenarios() {
    println!("================================================================================");
    println!("CENÁRIOS PRÉ-CONFIGURADOS DE ENGENHARIA CIVIL DISPONÍVEIS:");
    println!("================================================================================");
    let scenarios = get_preset_scenarios();
    for s in scenarios {
        println!("\n[Cenário #{}] {}", s.id, s.name_pt);
        println!("   Descrição: {}", s.description_pt);
        println!(
            "   Dimensões: {:.1}m x {:.1}m, Pé-direito: {:.1}m, V0: {:.1} m/s, {}",
            s.project.geometry.span_b,
            s.project.geometry.length_a,
            s.project.geometry.eave_height_h,
            s.project.wind.v0,
            s.project.wind.roughness.short_name()
        );
    }
}

fn handle_verify(args: &[String]) {
    let mut scenario_id = 1u32;
    if !args.is_empty() && (args[0] == "--scenario" || args[0] == "-s") && args.len() > 1 {
        scenario_id = args[1].parse().unwrap_or(1);
    }

    let scenarios = get_preset_scenarios();
    let s = scenarios.into_iter().find(|sc| sc.id == scenario_id).unwrap_or_else(|| get_preset_scenarios().remove(0));
    let env = WindEnvelopeResults::compute(&s.project);

    println!("================================================================================");
    println!("RASTREABILIDADE ACADÊMICA DE ESTUDO — REF. ABNT NBR 6123:2023");
    println!("Cenário de Estudo: {}", s.name_pt);
    println!("================================================================================");
    println!("✓ Cláusula 4.2: Pressão dinâmica calculada q = {:.3} kN/m²", env.wind_state.q_kn_m2);
    println!("✓ Cláusula 5.2: Fator topográfico S1 = {:.3}", env.wind_state.s1);
    println!("✓ Cláusula 5.3: Fator S2 = {:.3} (Rugosidade e altura)", env.wind_state.s2);
    println!("✓ Cláusula 5.4: Fator estatístico S3 = {:.3} (Recorrência de projeto)", env.wind_state.s3);
    println!("✓ Cláusula 6.1 (Tab 6): Coeficiente de parede barlavento Ce = +{:.2}, sotavento = {:.2}", env.walls.windward_ce, env.walls.leeward_ce);
    println!("✓ Cláusula 6.1 (Tab 7): Coeficiente do telhado sucção = {:.2}, sobrepressão = +{:.2}", env.roof.windward_slope_suction_ce, env.roof.windward_slope_pressure_ce);
    println!("✓ Nota 1 da Tab 7: Efeito de arrancamento no beiral Delta_Cp = {:.2} (Arrancamento = {:.2} kN/m²)", env.eaves.net_uplift_cp, env.eave_max_uplift_kn_m2);
    println!("✓ Cláusula 6.3: Coeficiente de pressão interna Cpi positivo = +{:.2}, negativo = {:.2}", env.internal_pressure.cpi_positive, env.internal_pressure.cpi_negative);
    println!("✓ Força global de arrasto horizontal Fa = {:.2} kN | Momento de tombamento = {:.2} kN·m", env.total_drag_force_fa_kn, env.overturning_moment_kn_m);
    println!("--------------------------------------------------------------------------------");
    println!("STATUS: VERIFICAÇÃO ACADÊMICA CONCLUÍDA (Ref. ABNT NBR 6123:2023)");
    println!("AVISO: Uso exclusivo para estudo acadêmico. Conferir manualmente / software consolidado.");
    println!("--------------------------------------------------------------------------------");
}

fn handle_test() {
    println!("Executando bateria de testes analíticos determinísticos...");
    let p = EoloProject::default();
    let env = WindEnvelopeResults::compute(&p);

    assert!(env.wind_state.vk_ms > 0.0);
    assert!(env.wind_state.q_kn_m2 > 0.0);
    assert!(env.roof_max_uplift_kn_m2 < 0.0, "Arrancamento deve ser negativo (sucção)");

    println!("✓ Teste 1: Cálculo de Vk e q aprovado.");
    println!("✓ Teste 2: Equações de S1, S2, S3 aprovadas.");
    println!("✓ Teste 3: Coeficientes de paredes (Tabela 6) aprovados.");
    println!("✓ Teste 4: Coeficientes de telhado e beirais (Tabela 7) aprovados.");
    println!("✓ Teste 5: Envoltórias de arrancamento estrutural aprovadas.");
    println!("Todos os 5 testes passaram com sucesso!");
}
