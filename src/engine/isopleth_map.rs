use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeteorologicalStation {
    pub id: u32,
    pub name: &'static str,
    pub uf: &'static str,
    pub latitude_deg: f64,
    pub longitude_deg: f64,
    pub altitude_m: f64,
    pub v0_isopleth: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrazilianStateInfo {
    pub uf: &'static str,
    pub name: &'static str,
    pub region: &'static str,
    pub capital: &'static str,
    pub v0_typical: f64,
    pub v0_min: f64,
    pub v0_max: f64,
}

pub const BRAZILIAN_STATES: &[BrazilianStateInfo] = &[
    BrazilianStateInfo { uf: "AC", name: "Acre", region: "Norte", capital: "Rio Branco", v0_typical: 30.0, v0_min: 30.0, v0_max: 30.0 },
    BrazilianStateInfo { uf: "AL", name: "Alagoas", region: "Nordeste", capital: "Maceió", v0_typical: 30.0, v0_min: 30.0, v0_max: 35.0 },
    BrazilianStateInfo { uf: "AP", name: "Amapá", region: "Norte", capital: "Macapá", v0_typical: 30.0, v0_min: 30.0, v0_max: 30.0 },
    BrazilianStateInfo { uf: "AM", name: "Amazonas", region: "Norte", capital: "Manaus", v0_typical: 30.0, v0_min: 30.0, v0_max: 30.0 },
    BrazilianStateInfo { uf: "BA", name: "Bahia", region: "Nordeste", capital: "Salvador", v0_typical: 30.0, v0_min: 30.0, v0_max: 35.0 },
    BrazilianStateInfo { uf: "CE", name: "Ceará", region: "Nordeste", capital: "Fortaleza", v0_typical: 30.0, v0_min: 30.0, v0_max: 30.0 },
    BrazilianStateInfo { uf: "DF", name: "Distrito Federal", region: "Centro-Oeste", capital: "Brasília", v0_typical: 35.0, v0_min: 35.0, v0_max: 35.0 },
    BrazilianStateInfo { uf: "ES", name: "Espírito Santo", region: "Sudeste", capital: "Vitória", v0_typical: 35.0, v0_min: 35.0, v0_max: 40.0 },
    BrazilianStateInfo { uf: "GO", name: "Goiás", region: "Centro-Oeste", capital: "Goiânia", v0_typical: 35.0, v0_min: 30.0, v0_max: 35.0 },
    BrazilianStateInfo { uf: "MA", name: "Maranhão", region: "Nordeste", capital: "São Luís", v0_typical: 30.0, v0_min: 30.0, v0_max: 30.0 },
    BrazilianStateInfo { uf: "MT", name: "Mato Grosso", region: "Centro-Oeste", capital: "Cuiabá", v0_typical: 30.0, v0_min: 30.0, v0_max: 35.0 },
    BrazilianStateInfo { uf: "MS", name: "Mato Grosso do Sul", region: "Centro-Oeste", capital: "Campo Grande", v0_typical: 40.0, v0_min: 35.0, v0_max: 45.0 },
    BrazilianStateInfo { uf: "MG", name: "Minas Gerais", region: "Sudeste", capital: "Belo Horizonte", v0_typical: 35.0, v0_min: 30.0, v0_max: 40.0 },
    BrazilianStateInfo { uf: "PA", name: "Pará", region: "Norte", capital: "Belém", v0_typical: 30.0, v0_min: 30.0, v0_max: 30.0 },
    BrazilianStateInfo { uf: "PB", name: "Paraíba", region: "Nordeste", capital: "João Pessoa", v0_typical: 30.0, v0_min: 30.0, v0_max: 30.0 },
    BrazilianStateInfo { uf: "PR", name: "Paraná", region: "Sul", capital: "Curitiba", v0_typical: 45.0, v0_min: 40.0, v0_max: 50.0 },
    BrazilianStateInfo { uf: "PE", name: "Pernambuco", region: "Nordeste", capital: "Recife", v0_typical: 30.0, v0_min: 30.0, v0_max: 30.0 },
    BrazilianStateInfo { uf: "PI", name: "Piauí", region: "Nordeste", capital: "Teresina", v0_typical: 30.0, v0_min: 30.0, v0_max: 30.0 },
    BrazilianStateInfo { uf: "RJ", name: "Rio de Janeiro", region: "Sudeste", capital: "Rio de Janeiro", v0_typical: 40.0, v0_min: 35.0, v0_max: 45.0 },
    BrazilianStateInfo { uf: "RN", name: "Rio Grande do Norte", region: "Nordeste", capital: "Natal", v0_typical: 30.0, v0_min: 30.0, v0_max: 30.0 },
    BrazilianStateInfo { uf: "RS", name: "Rio Grande do Sul", region: "Sul", capital: "Porto Alegre", v0_typical: 45.0, v0_min: 45.0, v0_max: 50.0 },
    BrazilianStateInfo { uf: "RO", name: "Rondônia", region: "Norte", capital: "Porto Velho", v0_typical: 30.0, v0_min: 30.0, v0_max: 30.0 },
    BrazilianStateInfo { uf: "RR", name: "Roraima", region: "Norte", capital: "Boa Vista", v0_typical: 30.0, v0_min: 30.0, v0_max: 30.0 },
    BrazilianStateInfo { uf: "SC", name: "Santa Catarina", region: "Sul", capital: "Florianópolis", v0_typical: 45.0, v0_min: 40.0, v0_max: 50.0 },
    BrazilianStateInfo { uf: "SP", name: "São Paulo", region: "Sudeste", capital: "São Paulo", v0_typical: 40.0, v0_min: 35.0, v0_max: 45.0 },
    BrazilianStateInfo { uf: "SE", name: "Sergipe", region: "Nordeste", capital: "Aracaju", v0_typical: 30.0, v0_min: 30.0, v0_max: 30.0 },
    BrazilianStateInfo { uf: "TO", name: "Tocantins", region: "Norte", capital: "Palmas", v0_typical: 30.0, v0_min: 30.0, v0_max: 35.0 },
];

/// 49 Estações Meteorológicas Oficiais da Aeronáutica (Tabela C.1 da NBR 6123:2023)
pub const NBR6123_STATIONS: &[MeteorologicalStation] = &[
    MeteorologicalStation { id: 1, name: "Afonsos", uf: "RJ", latitude_deg: -22.8667, longitude_deg: -43.3667, altitude_m: 31.0, v0_isopleth: 40.0 },
    MeteorologicalStation { id: 2, name: "Anápolis", uf: "GO", latitude_deg: -16.3667, longitude_deg: -48.9500, altitude_m: 1097.0, v0_isopleth: 35.0 },
    MeteorologicalStation { id: 3, name: "Amapá", uf: "AP", latitude_deg: 2.0667, longitude_deg: -50.5333, altitude_m: 10.0, v0_isopleth: 30.0 },
    MeteorologicalStation { id: 4, name: "Belém", uf: "PA", latitude_deg: -1.3833, longitude_deg: -48.4833, altitude_m: 16.0, v0_isopleth: 30.0 },
    MeteorologicalStation { id: 5, name: "Belo Horizonte (Pampulha)", uf: "MG", latitude_deg: -19.8500, longitude_deg: -43.9500, altitude_m: 789.0, v0_isopleth: 35.0 },
    MeteorologicalStation { id: 6, name: "Brasília", uf: "DF", latitude_deg: -15.8667, longitude_deg: -47.9167, altitude_m: 1061.0, v0_isopleth: 35.0 },
    MeteorologicalStation { id: 7, name: "Bagé", uf: "RS", latitude_deg: -31.3833, longitude_deg: -54.1167, altitude_m: 180.0, v0_isopleth: 45.0 },
    MeteorologicalStation { id: 8, name: "Boa Vista", uf: "RR", latitude_deg: 2.8333, longitude_deg: -60.7000, altitude_m: 140.0, v0_isopleth: 30.0 },
    MeteorologicalStation { id: 9, name: "Caravelas", uf: "BA", latitude_deg: -17.6333, longitude_deg: -39.2500, altitude_m: 4.0, v0_isopleth: 35.0 },
    MeteorologicalStation { id: 10, name: "Cachimbo", uf: "PA", latitude_deg: -9.3667, longitude_deg: -54.9000, altitude_m: 432.0, v0_isopleth: 30.0 },
    MeteorologicalStation { id: 11, name: "Cuiabá", uf: "MT", latitude_deg: -15.6500, longitude_deg: -56.1000, altitude_m: 182.0, v0_isopleth: 30.0 },
    MeteorologicalStation { id: 12, name: "Campinas (Viracopos)", uf: "SP", latitude_deg: -23.0000, longitude_deg: -47.1333, altitude_m: 648.0, v0_isopleth: 40.0 },
    MeteorologicalStation { id: 13, name: "Curitiba (Afonso Pena)", uf: "PR", latitude_deg: -25.5167, longitude_deg: -49.1833, altitude_m: 910.0, v0_isopleth: 45.0 },
    MeteorologicalStation { id: 14, name: "Campo Grande", uf: "MS", latitude_deg: -20.4667, longitude_deg: -54.6667, altitude_m: 552.0, v0_isopleth: 40.0 },
    MeteorologicalStation { id: 15, name: "Carolina", uf: "MA", latitude_deg: -7.3333, longitude_deg: -47.4333, altitude_m: 181.0, v0_isopleth: 30.0 },
    MeteorologicalStation { id: 16, name: "Cumbica (Guarulhos)", uf: "SP", latitude_deg: -23.4333, longitude_deg: -46.4667, altitude_m: 763.0, v0_isopleth: 40.0 },
    MeteorologicalStation { id: 17, name: "Fortaleza", uf: "CE", latitude_deg: -3.7833, longitude_deg: -38.5333, altitude_m: 25.0, v0_isopleth: 30.0 },
    MeteorologicalStation { id: 18, name: "Florianópolis (Hercílio Luz)", uf: "SC", latitude_deg: -27.6667, longitude_deg: -48.5500, altitude_m: 5.0, v0_isopleth: 45.0 },
    MeteorologicalStation { id: 19, name: "Foz do Iguaçu", uf: "PR", latitude_deg: -25.5167, longitude_deg: -54.5833, altitude_m: 180.0, v0_isopleth: 45.0 },
    MeteorologicalStation { id: 20, name: "Fernando de Noronha", uf: "PE", latitude_deg: -3.8500, longitude_deg: -32.4167, altitude_m: 45.0, v0_isopleth: 30.0 },
    MeteorologicalStation { id: 21, name: "Goiânia", uf: "GO", latitude_deg: -16.6333, longitude_deg: -49.2167, altitude_m: 747.0, v0_isopleth: 35.0 },
    MeteorologicalStation { id: 22, name: "Jacareacanga", uf: "PA", latitude_deg: -6.2667, longitude_deg: -57.7333, altitude_m: 110.0, v0_isopleth: 30.0 },
    MeteorologicalStation { id: 23, name: "Londrina", uf: "PR", latitude_deg: -23.3333, longitude_deg: -51.1333, altitude_m: 570.0, v0_isopleth: 45.0 },
    MeteorologicalStation { id: 24, name: "Lapa", uf: "PR", latitude_deg: -25.7667, longitude_deg: -49.7167, altitude_m: 908.0, v0_isopleth: 45.0 },
    MeteorologicalStation { id: 25, name: "Manaus (Eduardo Gomes)", uf: "AM", latitude_deg: -3.1500, longitude_deg: -59.9833, altitude_m: 84.0, v0_isopleth: 30.0 },
    MeteorologicalStation { id: 26, name: "Maceió", uf: "AL", latitude_deg: -9.5167, longitude_deg: -35.7833, altitude_m: 115.0, v0_isopleth: 30.0 },
    MeteorologicalStation { id: 27, name: "Natal", uf: "RN", latitude_deg: -5.9167, longitude_deg: -35.2500, altitude_m: 49.0, v0_isopleth: 30.0 },
    MeteorologicalStation { id: 28, name: "Ponta Porã", uf: "MS", latitude_deg: -22.5500, longitude_deg: -55.7000, altitude_m: 660.0, v0_isopleth: 45.0 },
    MeteorologicalStation { id: 29, name: "Parnaíba", uf: "PI", latitude_deg: -2.9000, longitude_deg: -41.7500, altitude_m: 5.0, v0_isopleth: 30.0 },
    MeteorologicalStation { id: 30, name: "Petrolina", uf: "PE", latitude_deg: -9.4000, longitude_deg: -40.5000, altitude_m: 376.0, v0_isopleth: 30.0 },
    MeteorologicalStation { id: 31, name: "Pirassununga", uf: "SP", latitude_deg: -21.9833, longitude_deg: -47.3500, altitude_m: 598.0, v0_isopleth: 40.0 },
    MeteorologicalStation { id: 32, name: "Porto Alegre (Salgado Filho)", uf: "RS", latitude_deg: -30.0000, longitude_deg: -51.1667, altitude_m: 4.0, v0_isopleth: 45.0 },
    MeteorologicalStation { id: 33, name: "Porto Nacional", uf: "TO", latitude_deg: -10.7000, longitude_deg: -48.4167, altitude_m: 290.0, v0_isopleth: 30.0 },
    MeteorologicalStation { id: 34, name: "Porto Velho", uf: "RO", latitude_deg: -8.7667, longitude_deg: -63.9000, altitude_m: 125.0, v0_isopleth: 30.0 },
    MeteorologicalStation { id: 35, name: "Recife (Guararapes)", uf: "PE", latitude_deg: -8.1333, longitude_deg: -34.9167, altitude_m: 11.0, v0_isopleth: 30.0 },
    MeteorologicalStation { id: 36, name: "Rio Branco", uf: "AC", latitude_deg: -9.9667, longitude_deg: -67.7833, altitude_m: 136.0, v0_isopleth: 30.0 },
    MeteorologicalStation { id: 37, name: "Rio de Janeiro (Santos Dumont)", uf: "RJ", latitude_deg: -22.9000, longitude_deg: -43.1667, altitude_m: 5.0, v0_isopleth: 40.0 },
    MeteorologicalStation { id: 38, name: "Santarém", uf: "PA", latitude_deg: -2.4333, longitude_deg: -54.7167, altitude_m: 12.0, v0_isopleth: 30.0 },
    MeteorologicalStation { id: 39, name: "São Luís", uf: "MA", latitude_deg: -2.5833, longitude_deg: -44.2333, altitude_m: 54.0, v0_isopleth: 30.0 },
    MeteorologicalStation { id: 40, name: "Salvador (Dois de Julho)", uf: "BA", latitude_deg: -12.9000, longitude_deg: -38.3333, altitude_m: 13.0, v0_isopleth: 30.0 },
    MeteorologicalStation { id: 41, name: "Santa Cruz", uf: "RJ", latitude_deg: -22.9333, longitude_deg: -43.7167, altitude_m: 4.0, v0_isopleth: 40.0 },
    MeteorologicalStation { id: 42, name: "São Paulo (Congonhas)", uf: "SP", latitude_deg: -23.6167, longitude_deg: -46.6500, altitude_m: 802.0, v0_isopleth: 40.0 },
    MeteorologicalStation { id: 43, name: "Santos", uf: "SP", latitude_deg: -23.9333, longitude_deg: -46.3000, altitude_m: 3.0, v0_isopleth: 40.0 },
    MeteorologicalStation { id: 44, name: "Santa Maria", uf: "RS", latitude_deg: -29.7167, longitude_deg: -53.7000, altitude_m: 85.0, v0_isopleth: 45.0 },
    MeteorologicalStation { id: 45, name: "Teresina", uf: "PI", latitude_deg: -5.0833, longitude_deg: -42.8167, altitude_m: 69.0, v0_isopleth: 30.0 },
    MeteorologicalStation { id: 46, name: "Uberlândia", uf: "MG", latitude_deg: -18.9167, longitude_deg: -48.2333, altitude_m: 923.0, v0_isopleth: 35.0 },
    MeteorologicalStation { id: 47, name: "Uruguaiana", uf: "RS", latitude_deg: -29.7833, longitude_deg: -57.0333, altitude_m: 74.0, v0_isopleth: 45.0 },
    MeteorologicalStation { id: 48, name: "Vitória", uf: "ES", latitude_deg: -20.2667, longitude_deg: -40.2833, altitude_m: 4.0, v0_isopleth: 35.0 },
    MeteorologicalStation { id: 49, name: "Vilhena", uf: "RO", latitude_deg: -12.7333, longitude_deg: -60.1333, altitude_m: 652.0, v0_isopleth: 30.0 },
];

pub fn get_state_by_uf(uf: &str) -> Option<&'static BrazilianStateInfo> {
    BRAZILIAN_STATES.iter().find(|s| s.uf.eq_ignore_ascii_case(uf))
}

pub fn get_station_by_id(id: u32) -> Option<&'static MeteorologicalStation> {
    NBR6123_STATIONS.iter().find(|s| s.id == id)
}

pub fn get_stations_by_uf(uf: &str) -> Vec<&'static MeteorologicalStation> {
    NBR6123_STATIONS
        .iter()
        .filter(|s| s.uf.eq_ignore_ascii_case(uf))
        .collect()
}

/// Sugestão de V0 com base em coordenadas geográficas aproximadas no Brasil
pub fn estimate_v0_from_coordinates(lat: f64, _lon: f64) -> f64 {
    // No Brasil, a latitude varia de cerca de +5° (Roraima) a -33° (Chuí/RS).
    // Ao sul do paralelo -23° (Sul de SP, PR, SC, RS, MS sul), os ventos atingem 45 a 50 m/s.
    // Entre -15° e -23° (SP, MG, RJ, GO, MS centro), ventos de 35 a 45 m/s.
    // Ao norte de -15° (Nordeste, Norte, Centro-Oeste norte), ventos de 30 a 35 m/s.
    if lat <= -26.0 {
        45.0 // PR, SC, RS
    } else if lat <= -22.0 {
        40.0 // SP, RJ, Sul de MG, MS
    } else if lat <= -15.0 {
        35.0 // DF, GO, MG norte, ES, BA sul
    } else {
        30.0 // Regiões Norte e Nordeste
    }
}
