use eolo_rs::core::project::EoloProject;
use eolo_rs::locale::{Language, Localizer};
use eolo_rs::ui::dialogs::{
    AbaSobre, AboutDialog, AuditDialog, ManualDialog, QuickCalcDialog, SettingsDialog, UnitsDialog,
};

#[test]
fn test_render_all_active_dialogs_bilingual() {
    let ctx = egui::Context::default();
    let mut project = EoloProject::default();

    let langs = [Language::PtBr, Language::EnUs];
    let abas = [
        AbaSobre::Sobre,
        AbaSobre::Historico,
        AbaSobre::Clc,
        AbaSobre::Copyright,
        AbaSobre::Eula,
        AbaSobre::Licencas,
        AbaSobre::Avisos,
        AbaSobre::Privacidade,
        AbaSobre::Release,
    ];

    let mut about = AboutDialog::default();
    about.open();
    let mut manual = ManualDialog::default();
    manual.open();
    let mut audit = AuditDialog::default();
    audit.open();
    let mut quick = QuickCalcDialog::default();
    quick.open();
    let mut units = UnitsDialog::default();
    units.open();
    let mut settings = SettingsDialog::default();
    settings.open();

    for lang in langs {
        let loc = Localizer::new(lang);
        for &dark in &[true, false] {
            for &aba in &abas {
                about.aba_ativa = aba;
                let mut out = ctx.run_ui(egui::RawInput::default(), |ctx| {
                    about.show(ctx, &loc, dark);
                });
                out.textures_delta.clear();
            }

            let mut out_m = ctx.run_ui(egui::RawInput::default(), |ctx| {
                manual.show(ctx, &loc, dark);
            });
            out_m.textures_delta.clear();

            let mut out_a = ctx.run_ui(egui::RawInput::default(), |ctx| {
                audit.show(ctx, &loc, dark);
            });
            out_a.textures_delta.clear();

            let mut out_q = ctx.run_ui(egui::RawInput::default(), |ctx| {
                quick.show(ctx, &loc, dark);
            });
            out_q.textures_delta.clear();

            let mut out_u = ctx.run_ui(egui::RawInput::default(), |ctx| {
                units.show(ctx, &loc, dark);
            });
            out_u.textures_delta.clear();

            let mut dark_mut = dark;
            let mut lang_mut = lang;
            let mut out_s = ctx.run_ui(egui::RawInput::default(), |ctx| {
                settings.show(ctx, &mut project, &mut dark_mut, &mut lang_mut);
            });
            out_s.textures_delta.clear();
        }
    }
}

#[test]
fn test_notices_contain_bibliographic_references_and_academic_disclaimers() {
    let notice_pt = include_str!("../src/assets/NOTICES/NOTICE_pt_BR.txt");
    let notice_en = include_str!("../src/assets/NOTICES/NOTICE_en_US.txt");

    for norm in ["ABNT NBR 6123:2023", "ABNT NBR 8800:2024", "ABNT NBR 6118:2023"] {
        assert!(
            notice_pt.contains(norm),
            "NOTICE_pt_BR.txt deve conter a referência bibliográfica {}",
            norm
        );
        assert!(
            notice_en.contains(norm),
            "NOTICE_en_US.txt must contain bibliographic reference {}",
            norm
        );
    }

    assert!(notice_pt.contains("ESTUDOS ACADÊMICOS"));
    assert!(notice_pt.contains("conferir manualmente"));
    assert!(notice_en.contains("ACADEMIC STUDY"));
    assert!(notice_en.contains("verify all inputs and outputs manually"));
}

#[test]
fn test_all_seven_asset_directories_contain_academic_non_production_disclaimers() {
    let pt_assets: &[(&str, &str)] = &[
        ("ABOUT/ABOUT_pt_BR.txt", include_str!("../src/assets/ABOUT/ABOUT_pt_BR.txt")),
        ("ABOUT/History_APP_pt_BR.txt", include_str!("../src/assets/ABOUT/History_APP_pt_BR.txt")),
        ("CLC/CLC_pt_BR - Eolo.txt", include_str!("../src/assets/CLC/CLC_pt_BR - Eolo.txt")),
        ("COPYRIGHT/AVISO_pt_BR.txt", include_str!("../src/assets/COPYRIGHT/AVISO DE COPYRIGHT E MARCA REGISTRA_pt_BR.txt")),
        ("EULA/EULA_pt_BR - Eolo.txt", include_str!("../src/assets/EULA/EULA_pt_BR - Eolo.txt")),
        ("NOTICES/NOTICE_pt_BR.txt", include_str!("../src/assets/NOTICES/NOTICE_pt_BR.txt")),
        ("PRIVACY_POLICY/Privacy_Policy_pt_BR.txt", include_str!("../src/assets/PRIVACY_POLICY/Privacy_Policy_pt_BR.txt")),
        ("RELEASE/RELEASE NOTES_pt_BR.txt", include_str!("../src/assets/RELEASE/RELEASE NOTES_pt_BR.txt")),
    ];

    let en_assets: &[(&str, &str)] = &[
        ("ABOUT/ABOUT_en_US.txt", include_str!("../src/assets/ABOUT/ABOUT_en_US.txt")),
        ("ABOUT/History_APP_en_US.txt", include_str!("../src/assets/ABOUT/History_APP_en_US.txt")),
        ("CLC/CLC_en_US - Eolo.txt", include_str!("../src/assets/CLC/CLC_en_US - Eolo.txt")),
        ("COPYRIGHT/COPYRIGHT_en_US.txt", include_str!("../src/assets/COPYRIGHT/COPYRIGHT AND TRADEMARK NOTICE_en_US.txt")),
        ("EULA/EULA_en_US - Eolo.txt", include_str!("../src/assets/EULA/EULA_en_US - Eolo.txt")),
        ("NOTICES/NOTICE_en_US.txt", include_str!("../src/assets/NOTICES/NOTICE_en_US.txt")),
        ("PRIVACY_POLICY/Privacy_Policy_en_US.txt", include_str!("../src/assets/PRIVACY_POLICY/Privacy_Policy_en_US.txt")),
        ("RELEASE/RELEASE NOTES_en_US.txt", include_str!("../src/assets/RELEASE/RELEASE NOTES_en_US.txt")),
    ];

    for (name, txt) in pt_assets {
        let lower = txt.to_lowercase();
        assert!(
            lower.contains("acadêmic"),
            "{} must state academic study purpose",
            name
        );
        assert!(
            lower.contains("erro"),
            "{} must warn about possible errors",
            name
        );
        assert!(
            lower.contains("produção"),
            "{} must state non-production restriction",
            name
        );
        assert!(
            lower.contains("manual") && lower.contains("consolidad"),
            "{} must require manual and consolidated-tool verification",
            name
        );
    }

    for (name, txt) in en_assets {
        let lower = txt.to_lowercase();
        assert!(
            lower.contains("academic"),
            "{} must state academic study purpose",
            name
        );
        assert!(
            lower.contains("error"),
            "{} must warn about possible errors",
            name
        );
        assert!(
            lower.contains("production"),
            "{} must state non-production restriction",
            name
        );
        assert!(
            lower.contains("manual") && lower.contains("consolidat"),
            "{} must require manual and consolidated-tool verification",
            name
        );
    }
}
