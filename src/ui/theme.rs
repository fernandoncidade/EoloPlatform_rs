use egui::{Color32, Context, CornerRadius, Stroke, Style, Visuals};

/// Paleta de cores técnica do Eolo Platform (Estilo Engenharia Estrutural & Aeroelasticidade)
/// com suporte dinâmico completo em tempo de execução aos Temas Escuro e Claro.
pub struct EoloTheme;

impl EoloTheme {
    // Constantes estáticas de referência (compatibilidade)
    pub const ACCENT_BLUE: Color32 = Color32::from_rgb(21, 101, 192);
    pub const OVERPRESSURE_RED: Color32 = Color32::from_rgb(244, 81, 30);
    pub const SUCTION_CYAN: Color32 = Color32::from_rgb(0, 180, 216);
    pub const EAVE_MAGENTA: Color32 = Color32::from_rgb(171, 71, 188);
    pub const WIND_GOLD: Color32 = Color32::from_rgb(255, 193, 7);
    pub const STEEL_GRAY: Color32 = Color32::from_rgb(144, 164, 174);
    pub const SUCCESS_GREEN: Color32 = Color32::from_rgb(46, 125, 50);

    // =========================================================================
    // Cores Dinâmicas Sensíveis ao Tema (Dark Mode vs. Light Mode)
    // =========================================================================

    /// Azul de destaque principal para cabeçalhos, equações e valores principais
    pub fn accent_blue(dark_mode: bool) -> Color32 {
        if dark_mode {
            Color32::from_rgb(100, 181, 246) // Azul celeste claro de alto contraste no escuro
        } else {
            Color32::from_rgb(13, 71, 161) // Azul marinho técnico forte no claro
        }
    }

    /// Azul secundário para subtítulos e seções
    pub fn secondary_blue(dark_mode: bool) -> Color32 {
        if dark_mode {
            Color32::from_rgb(129, 212, 250)
        } else {
            Color32::from_rgb(25, 118, 210)
        }
    }

    /// Vermelho/Laranja para sobrepressão (+Cp / +w) e erros
    pub fn overpressure_red(dark_mode: bool) -> Color32 {
        if dark_mode {
            Color32::from_rgb(255, 112, 67)
        } else {
            Color32::from_rgb(198, 40, 40)
        }
    }

    /// Ciano/Azul para sucção e arrancamento (-Cp / -w)
    pub fn suction_cyan(dark_mode: bool) -> Color32 {
        if dark_mode {
            Color32::from_rgb(38, 198, 218)
        } else {
            Color32::from_rgb(0, 105, 148)
        }
    }

    /// Magenta/Roxo para arrancamento crítico em beirais
    pub fn eave_magenta(dark_mode: bool) -> Color32 {
        if dark_mode {
            Color32::from_rgb(240, 98, 146)
        } else {
            Color32::from_rgb(142, 36, 170)
        }
    }

    /// Dourado/Âmbar para direção do vento e alertas técnicos
    pub fn wind_gold(dark_mode: bool) -> Color32 {
        if dark_mode {
            Color32::from_rgb(255, 213, 79)
        } else {
            Color32::from_rgb(216, 67, 21) // Âmbar escuro legível em fundo claro
        }
    }

    /// Cinza metálico para perfis de aço e descrições técnicas
    pub fn steel_gray(dark_mode: bool) -> Color32 {
        if dark_mode {
            Color32::from_rgb(176, 190, 197)
        } else {
            Color32::from_rgb(69, 90, 100)
        }
    }

    /// Verde para conformidade normativa e confirmações
    pub fn success_green(dark_mode: bool) -> Color32 {
        if dark_mode {
            Color32::from_rgb(102, 187, 106)
        } else {
            Color32::from_rgb(27, 94, 32)
        }
    }

    /// Cor de texto primário garantida para qualquer widget ou canvas
    pub fn text_primary(dark_mode: bool) -> Color32 {
        if dark_mode {
            Color32::from_rgb(235, 240, 245)
        } else {
            Color32::from_rgb(20, 24, 31)
        }
    }

    /// Cor de texto secundário (notas, legendas auxiliares, descrições)
    pub fn text_secondary(dark_mode: bool) -> Color32 {
        if dark_mode {
            Color32::from_rgb(165, 175, 190)
        } else {
            Color32::from_rgb(75, 85, 100)
        }
    }

    /// Cor de fundo para cartões de resumo e badges na toolbar
    pub fn card_bg(dark_mode: bool) -> Color32 {
        if dark_mode {
            Color32::from_rgb(28, 33, 42)
        } else {
            Color32::from_rgb(236, 241, 248)
        }
    }

    /// Cor de borda para cartões de resumo e molduras finas
    pub fn card_stroke(dark_mode: bool) -> Color32 {
        if dark_mode {
            Color32::from_rgb(58, 68, 84)
        } else {
            Color32::from_rgb(195, 205, 220)
        }
    }

    /// Cor para tabelas monoespaçadas no Memorial de Cálculo
    pub fn table_monospace(dark_mode: bool) -> Color32 {
        if dark_mode {
            Color32::from_rgb(180, 215, 245)
        } else {
            Color32::from_rgb(25, 60, 105)
        }
    }

    // =========================================================================
    // Cores Específicas do Canvas 2D/3D na GPU (Aba 4)
    // =========================================================================

    pub fn canvas_bg(dark_mode: bool) -> Color32 {
        if dark_mode {
            Color32::from_rgb(14, 17, 23)
        } else {
            Color32::from_rgb(247, 250, 254)
        }
    }

    pub fn canvas_border(dark_mode: bool) -> Color32 {
        if dark_mode {
            Color32::from_rgb(50, 60, 76)
        } else {
            Color32::from_rgb(180, 195, 215)
        }
    }

    pub fn canvas_grid(dark_mode: bool) -> Color32 {
        if dark_mode {
            Color32::from_rgb(35, 43, 56)
        } else {
            Color32::from_rgb(218, 226, 236)
        }
    }

    pub fn ground_line(dark_mode: bool) -> Color32 {
        if dark_mode {
            Color32::from_rgb(110, 125, 145)
        } else {
            Color32::from_rgb(90, 105, 120)
        }
    }

    pub fn wireframe_main(dark_mode: bool) -> Color32 {
        if dark_mode {
            Color32::from_rgb(180, 205, 230)
        } else {
            Color32::from_rgb(35, 55, 80)
        }
    }

    pub fn wireframe_ridge(dark_mode: bool) -> Color32 {
        if dark_mode {
            Color32::from_rgb(210, 230, 255)
        } else {
            Color32::from_rgb(21, 101, 192)
        }
    }

    pub fn wireframe_purlin(dark_mode: bool) -> Color32 {
        if dark_mode {
            Color32::from_rgb(105, 125, 145)
        } else {
            Color32::from_rgb(110, 130, 155)
        }
    }

    /// Aplica os estilos e cores de tema dinâmico (Escuro ou Claro) a todo o Contexto egui.
    pub fn apply(ctx: &Context, dark_mode: bool) {
        ctx.set_theme(if dark_mode {
            egui::Theme::Dark
        } else {
            egui::Theme::Light
        });

        let mut visuals = if dark_mode {
            let mut v = Visuals::dark();
            v.override_text_color = None;
            v.panel_fill = Color32::from_rgb(20, 23, 29);
            v.window_fill = Color32::from_rgb(25, 29, 36);
            v.extreme_bg_color = Color32::from_rgb(14, 16, 21);
            v.faint_bg_color = Color32::from_rgb(28, 33, 42);
            v.code_bg_color = Color32::from_rgb(30, 35, 44);

            v.widgets.noninteractive.bg_fill = Color32::from_rgb(26, 30, 38);
            v.widgets.noninteractive.weak_bg_fill = Color32::from_rgb(26, 30, 38);
            v.widgets.noninteractive.bg_stroke = Stroke::new(1.0, Color32::from_rgb(52, 60, 74));
            v.widgets.noninteractive.fg_stroke = Stroke::new(1.0, Color32::from_rgb(228, 233, 240));

            v.widgets.inactive.bg_fill = Color32::from_rgb(36, 42, 54);
            v.widgets.inactive.weak_bg_fill = Color32::from_rgb(34, 40, 50);
            v.widgets.inactive.bg_stroke = Stroke::new(1.0, Color32::from_rgb(64, 74, 92));
            v.widgets.inactive.fg_stroke = Stroke::new(1.0, Color32::from_rgb(230, 235, 242));

            v.widgets.hovered.bg_fill = Color32::from_rgb(48, 58, 76);
            v.widgets.hovered.weak_bg_fill = Color32::from_rgb(48, 58, 76);
            v.widgets.hovered.bg_stroke = Stroke::new(1.0, Color32::from_rgb(100, 181, 246));
            v.widgets.hovered.fg_stroke = Stroke::new(1.5, Color32::WHITE);

            v.widgets.active.bg_fill = Color32::from_rgb(25, 118, 210);
            v.widgets.active.weak_bg_fill = Color32::from_rgb(25, 118, 210);
            v.widgets.active.bg_stroke = Stroke::new(1.0, Color32::from_rgb(144, 202, 249));
            v.widgets.active.fg_stroke = Stroke::new(2.0, Color32::WHITE);

            v.widgets.open.bg_fill = Color32::from_rgb(42, 50, 66);
            v.widgets.open.weak_bg_fill = Color32::from_rgb(42, 50, 66);
            v.widgets.open.bg_stroke = Stroke::new(1.0, Color32::from_rgb(100, 181, 246));
            v.widgets.open.fg_stroke = Stroke::new(1.5, Color32::WHITE);

            v.selection.bg_fill = Color32::from_rgb(25, 118, 210);
            v.selection.stroke = Stroke::new(1.0, Color32::WHITE);
            v.window_stroke = Stroke::new(1.0, Color32::from_rgb(55, 65, 82));
            v
        } else {
            let mut v = Visuals::light();
            v.override_text_color = None;
            v.panel_fill = Color32::from_rgb(244, 246, 250);
            v.window_fill = Color32::from_rgb(252, 253, 255);
            v.extreme_bg_color = Color32::from_rgb(255, 255, 255);
            v.faint_bg_color = Color32::from_rgb(235, 240, 247);
            v.code_bg_color = Color32::from_rgb(232, 238, 246);

            v.widgets.noninteractive.bg_fill = Color32::from_rgb(238, 242, 248);
            v.widgets.noninteractive.weak_bg_fill = Color32::from_rgb(238, 242, 248);
            v.widgets.noninteractive.bg_stroke = Stroke::new(1.0, Color32::from_rgb(198, 208, 222));
            v.widgets.noninteractive.fg_stroke = Stroke::new(1.0, Color32::from_rgb(22, 27, 34));

            v.widgets.inactive.bg_fill = Color32::from_rgb(226, 232, 242);
            v.widgets.inactive.weak_bg_fill = Color32::from_rgb(230, 236, 245);
            v.widgets.inactive.bg_stroke = Stroke::new(1.0, Color32::from_rgb(180, 192, 210));
            v.widgets.inactive.fg_stroke = Stroke::new(1.0, Color32::from_rgb(20, 25, 32));

            v.widgets.hovered.bg_fill = Color32::from_rgb(210, 224, 244);
            v.widgets.hovered.weak_bg_fill = Color32::from_rgb(210, 224, 244);
            v.widgets.hovered.bg_stroke = Stroke::new(1.0, Color32::from_rgb(21, 101, 192));
            v.widgets.hovered.fg_stroke = Stroke::new(1.5, Color32::from_rgb(10, 15, 25));

            v.widgets.active.bg_fill = Color32::from_rgb(185, 212, 248);
            v.widgets.active.weak_bg_fill = Color32::from_rgb(195, 218, 250);
            v.widgets.active.bg_stroke = Stroke::new(1.0, Color32::from_rgb(21, 101, 192));
            v.widgets.active.fg_stroke = Stroke::new(2.0, Color32::from_rgb(10, 15, 25));

            v.widgets.open.bg_fill = Color32::from_rgb(215, 228, 246);
            v.widgets.open.weak_bg_fill = Color32::from_rgb(215, 228, 246);
            v.widgets.open.bg_stroke = Stroke::new(1.0, Color32::from_rgb(21, 101, 192));
            v.widgets.open.fg_stroke = Stroke::new(1.5, Color32::from_rgb(10, 15, 25));

            v.selection.bg_fill = Color32::from_rgb(185, 212, 248);
            v.selection.stroke = Stroke::new(1.0, Color32::from_rgb(10, 15, 25));
            v.window_stroke = Stroke::new(1.0, Color32::from_rgb(185, 198, 215));
            v
        };

        visuals.window_corner_radius = CornerRadius::same(6);
        visuals.menu_corner_radius = CornerRadius::same(6);
        visuals.widgets.noninteractive.corner_radius = CornerRadius::same(4);
        visuals.widgets.inactive.corner_radius = CornerRadius::same(4);
        visuals.widgets.hovered.corner_radius = CornerRadius::same(4);
        visuals.widgets.active.corner_radius = CornerRadius::same(4);

        ctx.set_visuals(visuals);

        let configure_style = |s: &mut Style| {
            s.spacing.item_spacing = egui::vec2(8.0, 6.0);
            s.spacing.button_padding = egui::vec2(8.0, 4.0);
            s.spacing.scroll.foreground_color = false;
            s.spacing.scroll.floating = false;
            s.spacing.scroll.bar_width = 10.0;
            s.spacing.scroll.handle_min_length = 24.0;
            s.spacing.scroll.bar_inner_margin = 2.0;
            s.spacing.scroll.bar_outer_margin = 2.0;
        };
        ctx.style_mut_of(egui::Theme::Dark, configure_style);
        ctx.style_mut_of(egui::Theme::Light, configure_style);
    }
}
