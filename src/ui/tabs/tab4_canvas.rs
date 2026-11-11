use egui::{Color32, Pos2, Rect, RichText, Sense, Stroke, Ui};

use crate::core::project::EoloProject;
use crate::core::types::RoofTypology;
use crate::engine::envelopes::WindEnvelopeResults;
use crate::engine::math3d::{draw_arrow_3d, draw_line_3d, OrbitCamera, Vec3};
use crate::locale::Localizer;
use crate::ui::theme::EoloTheme;

pub struct CanvasTabState {
    pub camera: OrbitCamera,
    pub is_2d_section_mode: bool,
    pub show_purlins: bool,
    pub show_vectors: bool,
    pub vector_scale: f32,
    pub wind_incidence_angle_deg: f32,
}

impl Default for CanvasTabState {
    fn default() -> Self {
        Self {
            camera: OrbitCamera::default(),
            is_2d_section_mode: false,
            show_purlins: true,
            show_vectors: true,
            vector_scale: 1.0,
            wind_incidence_angle_deg: 90.0, // Vento transversal padrão
        }
    }
}

pub fn render_tab_canvas(
    ui: &mut Ui,
    project: &mut EoloProject,
    env: &WindEnvelopeResults,
    state: &mut CanvasTabState,
    localizer: &Localizer,
) {
    let dark = ui.visuals().dark_mode;

    ui.horizontal_wrapped(|ui| {
        ui.heading(
            RichText::new(localizer.tr(
                "4. Diagrama de Cargas & Wireframe 3D na GPU (egui)",
                "4. Load Diagrams & 3D GPU Wireframe (egui)",
            ))
            .color(EoloTheme::text_primary(dark))
            .strong(),
        );

        ui.add_space(12.0);

        let view_toggle_label = if state.is_2d_section_mode {
            localizer.tr("👁️ Vista 3D Interativa", "👁️ Interactive 3D View")
        } else {
            localizer.tr("📐 Corte Transversal 2D", "📐 2D Cross-Section")
        };
        if ui.button(view_toggle_label).clicked() {
            state.is_2d_section_mode = !state.is_2d_section_mode;
        }

        if ui
            .button(localizer.tr("↺ Resetar Câmera", "↺ Reset Camera"))
            .clicked()
        {
            let center = Vec3::new(
                (project.geometry.span_b * 0.5) as f32,
                (project.geometry.mean_height() * 0.5) as f32,
                (project.geometry.length_a * 0.5) as f32,
            );
            state.camera.reset_view(center, project.geometry.span_b as f32);
        }
    });

    ui.horizontal_wrapped(|ui| {
        ui.label(localizer.tr("Incidência do Vento (α):", "Wind Incidence (α):"));
        ui.add(egui::Slider::new(&mut state.wind_incidence_angle_deg, 0.0..=360.0).suffix("°"));

        if ui
            .button(localizer.tr("Transversal (90°)", "Transverse (90°)"))
            .clicked()
        {
            state.wind_incidence_angle_deg = 90.0;
        }
        if ui
            .button(localizer.tr("Longitudinal (0°)", "Longitudinal (0°)"))
            .clicked()
        {
            state.wind_incidence_angle_deg = 0.0;
        }

        ui.add_space(8.0);
        ui.checkbox(
            &mut state.show_vectors,
            localizer.tr(
                "Flechas de Carga (Sobrepressão/Sucção)",
                "Load Vectors (Overpressure/Suction)",
            ),
        );
        ui.checkbox(
            &mut state.show_purlins,
            localizer.tr("Terças & Longarinas", "Purlins & Girts"),
        );

        ui.add_space(8.0);
        ui.label(localizer.tr("Escala Flechas:", "Vector Scale:"));
        ui.add(egui::Slider::new(&mut state.vector_scale, 0.2..=3.0).text("x"));
    });

    // Legenda de Cores Dinâmica Sensível ao Tema
    ui.horizontal_wrapped(|ui| {
        ui.label(
            RichText::new(localizer.tr(
                "■ Sobrepressão (+Cp)",
                "■ Overpressure (+Cp)",
            ))
            .color(EoloTheme::overpressure_red(dark))
            .strong(),
        );
        ui.label(
            RichText::new(localizer.tr(
                "■ Sucção / Arrancamento (-Cp)",
                "■ Suction / Uplift (-Cp)",
            ))
            .color(EoloTheme::suction_cyan(dark))
            .strong(),
        );
        ui.label(
            RichText::new(localizer.tr(
                "■ Arrancamento no Beiral",
                "■ Eave Overhang Uplift",
            ))
            .color(EoloTheme::eave_magenta(dark))
            .strong(),
        );
        ui.label(
            RichText::new(localizer.tr("■ Direção do Vento", "■ Wind Direction"))
                .color(EoloTheme::wind_gold(dark))
                .strong(),
        );
        ui.label(
            RichText::new(localizer.tr("■ Perfis de Aço", "■ Steel Framing"))
                .color(EoloTheme::steel_gray(dark))
                .strong(),
        );
    });

    ui.separator();

    // Área do Canvas interativo
    let available_size = ui.available_size();
    let (response, painter) = ui.allocate_painter(available_size, Sense::drag());
    let rect = response.rect;

    // Fundo e borda do canvas adaptados dinamicamente ao Tema Escuro / Claro
    painter.rect(
        rect,
        6.0,
        EoloTheme::canvas_bg(dark),
        Stroke::new(1.0, EoloTheme::canvas_border(dark)),
        egui::StrokeKind::Outside,
    );

    if state.is_2d_section_mode {
        render_2d_cross_section(&painter, rect, project, env, dark, localizer);
    } else {
        state.camera.handle_input(ui, &response);
        render_3d_wireframe(&painter, rect, &state.camera, project, env, state, dark);
    }
}

/// Renderiza o pórtico transversal em 2D com cotas e diagramas de carga nas barras
fn render_2d_cross_section(
    painter: &egui::Painter,
    rect: Rect,
    project: &EoloProject,
    env: &WindEnvelopeResults,
    dark: bool,
    localizer: &Localizer,
) {
    let b = project.geometry.span_b as f32;
    let h_eave = project.geometry.eave_height_h as f32;
    let h_ridge = project.geometry.total_ridge_height() as f32;
    let eave_x = project.geometry.eave_overhang_x as f32;

    let margin = 70.0;
    let plot_w = (rect.width() - margin * 2.0).max(50.0);
    let plot_h = (rect.height() - margin * 2.0).max(50.0);

    let scale_x = plot_w / (b + eave_x * 2.0 + 8.0);
    let scale_y = plot_h / (h_ridge + 6.0);
    let scale = scale_x.min(scale_y);

    let origin_x = rect.left() + margin + 4.0 * scale;
    let origin_y = rect.bottom() - margin;

    let to_screen = |x: f32, y: f32| -> Pos2 {
        Pos2::new(origin_x + x * scale, origin_y - y * scale)
    };

    // Linha do solo
    let ground_p1 = to_screen(-eave_x - 3.0, 0.0);
    let ground_p2 = to_screen(b + eave_x + 3.0, 0.0);
    painter.line_segment(
        [ground_p1, ground_p2],
        Stroke::new(1.5, EoloTheme::ground_line(dark)),
    );

    // Pontos do pórtico
    let p_col_base_left = to_screen(0.0, 0.0);
    let p_col_top_left = to_screen(0.0, h_eave);
    let p_eave_left = to_screen(-eave_x, h_eave);

    let p_ridge = to_screen(b * 0.5, h_ridge);

    let p_col_base_right = to_screen(b, 0.0);
    let p_col_top_right = to_screen(b, h_eave);
    let p_eave_right = to_screen(b + eave_x, h_eave);

    let frame_stroke = Stroke::new(3.5, EoloTheme::wireframe_main(dark));

    // Pilares
    painter.line_segment([p_col_base_left, p_col_top_left], frame_stroke);
    painter.line_segment([p_col_base_right, p_col_top_right], frame_stroke);

    // Beirais
    painter.line_segment([p_eave_left, p_col_top_left], frame_stroke);
    painter.line_segment([p_col_top_right, p_eave_right], frame_stroke);

    // Vigas da cobertura
    match project.geometry.typology {
        RoofTypology::DuoPitch | RoofTypology::ShedIndustrial => {
            painter.line_segment([p_col_top_left, p_ridge], frame_stroke);
            painter.line_segment([p_ridge, p_col_top_right], frame_stroke);
        }
        RoofTypology::MonoPitch => {
            painter.line_segment([p_col_top_left, p_col_top_right], frame_stroke);
        }
        RoofTypology::CurvedArch => {
            let num_pts = 20;
            for i in 0..num_pts {
                let t1 = i as f32 / num_pts as f32;
                let t2 = (i + 1) as f32 / num_pts as f32;
                let x1 = t1 * b;
                let x2 = t2 * b;
                let y1 = h_eave + 4.0 * (h_ridge - h_eave) * t1 * (1.0 - t1);
                let y2 = h_eave + 4.0 * (h_ridge - h_eave) * t2 * (1.0 - t2);
                painter.line_segment([to_screen(x1, y1), to_screen(x2, y2)], frame_stroke);
            }
        }
        RoofTypology::CanopyIsolated => {
            painter.line_segment([p_col_top_left, p_col_top_right], frame_stroke);
        }
    }

    // Indicador da Direção do Vento
    let wind_color = EoloTheme::wind_gold(dark);
    let wind_arrow_start = to_screen(-eave_x - 4.5, h_eave * 0.5);
    let wind_arrow_end = to_screen(-eave_x - 1.5, h_eave * 0.5);
    painter.line_segment([wind_arrow_start, wind_arrow_end], Stroke::new(4.0, wind_color));
    painter.text(
        Pos2::new(wind_arrow_start.x, wind_arrow_start.y - 18.0),
        egui::Align2::LEFT_BOTTOM,
        format!(
            "{} Vk = {:.1} m/s",
            localizer.tr("VENTO", "WIND"),
            env.wind_state.vk_ms
        ),
        egui::FontId::proportional(13.0),
        wind_color,
    );

    // Flechas de carga no pilar de barlavento (Sobrepressão para dentro)
    let press_color = EoloTheme::overpressure_red(dark);
    let arrow_len_col = 30.0;
    for i in 1..=4 {
        let frac = i as f32 / 5.0;
        let y_pos = frac * h_eave;
        let p_surf = to_screen(0.0, y_pos);
        let p_out = Pos2::new(p_surf.x - arrow_len_col, p_surf.y);
        painter.line_segment([p_out, p_surf], Stroke::new(2.0, press_color));
    }
    painter.text(
        Pos2::new(
            p_col_base_left.x - 35.0,
            (p_col_base_left.y + p_col_top_left.y) * 0.5,
        ),
        egui::Align2::RIGHT_CENTER,
        format!("+w = {:.2} kN/m", env.frame_column_windward_kn_m),
        egui::FontId::proportional(11.0),
        press_color,
    );

    // Flechas de arrancamento no telhado de barlavento (Sucção puxando para fora)
    let suct_color = EoloTheme::suction_cyan(dark);
    let arrow_len_roof = 35.0;
    for i in 1..=3 {
        let frac = i as f32 / 4.0;
        let x_pos = frac * (b * 0.5);
        let y_pos = h_eave + frac * (h_ridge - h_eave);
        let p_surf = to_screen(x_pos, y_pos);
        let p_out = Pos2::new(p_surf.x - 10.0, p_surf.y - arrow_len_roof);
        painter.line_segment([p_surf, p_out], Stroke::new(2.0, suct_color));
    }
    painter.text(
        Pos2::new(
            (p_col_top_left.x + p_ridge.x) * 0.5,
            (p_col_top_left.y + p_ridge.y) * 0.5 - 45.0,
        ),
        egui::Align2::CENTER_BOTTOM,
        format!(
            "-w = {:.2} kN/m ({})",
            env.frame_rafter_windward_kn_m,
            localizer.tr("Arrancamento", "Uplift")
        ),
        egui::FontId::proportional(12.0),
        suct_color,
    );

    // Cota do Vão Livre b (com cor de texto adaptativa para legibilidade perfeita no Claro e Escuro)
    let dim_y = origin_y + 25.0;
    let dim_p1 = Pos2::new(to_screen(0.0, 0.0).x, dim_y);
    let dim_p2 = Pos2::new(to_screen(b, 0.0).x, dim_y);
    painter.line_segment(
        [dim_p1, dim_p2],
        Stroke::new(1.2, EoloTheme::text_secondary(dark)),
    );
    painter.text(
        Pos2::new((dim_p1.x + dim_p2.x) * 0.5, dim_y + 4.0),
        egui::Align2::CENTER_TOP,
        format!("{} b = {:.2} m", localizer.tr("Vão", "Span"), b),
        egui::FontId::proportional(12.0),
        EoloTheme::text_primary(dark),
    );
}

/// Renderiza o modelo wireframe 3D tridimensional com vetores de vento e sucção na GPU
fn render_3d_wireframe(
    painter: &egui::Painter,
    screen: Rect,
    camera: &OrbitCamera,
    project: &EoloProject,
    env: &WindEnvelopeResults,
    state: &CanvasTabState,
    dark: bool,
) {
    let b = project.geometry.span_b as f32;
    let a = project.geometry.length_a as f32;
    let h_eave = project.geometry.eave_height_h as f32;
    let h_ridge = project.geometry.total_ridge_height() as f32;
    let eave_x = project.geometry.eave_overhang_x as f32;
    let frame_s = project.geometry.frame_spacing as f32;

    let num_frames = ((a / frame_s).round() as usize).max(1);
    let actual_spacing = a / num_frames as f32;

    // Grade do piso (Ground Grid) adaptada ao Tema Escuro/Claro
    let grid_stroke = Stroke::new(0.8, EoloTheme::canvas_grid(dark));
    let grid_step = 5.0;
    let mut gx = -10.0;
    while gx <= b + 10.0 {
        draw_line_3d(
            painter,
            camera,
            screen,
            Vec3::new(gx, 0.0, -5.0),
            Vec3::new(gx, 0.0, a + 5.0),
            grid_stroke,
        );
        gx += grid_step;
    }
    let mut gz = -5.0;
    while gz <= a + 5.0 {
        draw_line_3d(
            painter,
            camera,
            screen,
            Vec3::new(-10.0, 0.0, gz),
            Vec3::new(b + 10.0, 0.0, gz),
            grid_stroke,
        );
        gz += grid_step;
    }

    let frame_stroke = Stroke::new(2.5, EoloTheme::wireframe_main(dark));
    let ridge_stroke = Stroke::new(2.0, EoloTheme::wireframe_ridge(dark));
    let purlin_stroke = Stroke::new(1.0, EoloTheme::wireframe_purlin(dark));

    // Desenho de cada pórtico transversal
    for i in 0..=num_frames {
        let z = i as f32 * actual_spacing;

        let col_base_l = Vec3::new(0.0, 0.0, z);
        let col_top_l = Vec3::new(0.0, h_eave, z);
        let col_base_r = Vec3::new(b, 0.0, z);
        let col_top_r = Vec3::new(b, h_eave, z);

        let eave_l = Vec3::new(-eave_x, h_eave, z);
        let eave_r = Vec3::new(b + eave_x, h_eave, z);
        let ridge = Vec3::new(b * 0.5, h_ridge, z);

        draw_line_3d(painter, camera, screen, col_base_l, col_top_l, frame_stroke);
        draw_line_3d(painter, camera, screen, col_base_r, col_top_r, frame_stroke);
        draw_line_3d(painter, camera, screen, eave_l, col_top_l, frame_stroke);
        draw_line_3d(painter, camera, screen, col_top_r, eave_r, frame_stroke);

        match project.geometry.typology {
            RoofTypology::DuoPitch | RoofTypology::ShedIndustrial => {
                draw_line_3d(painter, camera, screen, col_top_l, ridge, frame_stroke);
                draw_line_3d(painter, camera, screen, ridge, col_top_r, frame_stroke);
            }
            RoofTypology::MonoPitch => {
                draw_line_3d(painter, camera, screen, col_top_l, col_top_r, frame_stroke);
            }
            RoofTypology::CurvedArch => {
                let n_seg = 10;
                for s in 0..n_seg {
                    let t1 = s as f32 / n_seg as f32;
                    let t2 = (s + 1) as f32 / n_seg as f32;
                    let y1 = h_eave + 4.0 * (h_ridge - h_eave) * t1 * (1.0 - t1);
                    let y2 = h_eave + 4.0 * (h_ridge - h_eave) * t2 * (1.0 - t2);
                    draw_line_3d(
                        painter,
                        camera,
                        screen,
                        Vec3::new(t1 * b, y1, z),
                        Vec3::new(t2 * b, y2, z),
                        frame_stroke,
                    );
                }
            }
            RoofTypology::CanopyIsolated => {
                draw_line_3d(painter, camera, screen, col_top_l, col_top_r, frame_stroke);
            }
        }
    }

    // Linhas longitudinais: Cumeeira e calhas
    draw_line_3d(
        painter,
        camera,
        screen,
        Vec3::new(b * 0.5, h_ridge, 0.0),
        Vec3::new(b * 0.5, h_ridge, a),
        ridge_stroke,
    );
    draw_line_3d(
        painter,
        camera,
        screen,
        Vec3::new(0.0, h_eave, 0.0),
        Vec3::new(0.0, h_eave, a),
        ridge_stroke,
    );
    draw_line_3d(
        painter,
        camera,
        screen,
        Vec3::new(b, h_eave, 0.0),
        Vec3::new(b, h_eave, a),
        ridge_stroke,
    );
    draw_line_3d(
        painter,
        camera,
        screen,
        Vec3::new(-eave_x, h_eave, 0.0),
        Vec3::new(-eave_x, h_eave, a),
        ridge_stroke,
    );
    draw_line_3d(
        painter,
        camera,
        screen,
        Vec3::new(b + eave_x, h_eave, 0.0),
        Vec3::new(b + eave_x, h_eave, a),
        ridge_stroke,
    );

    // Terças longitudinais
    if state.show_purlins {
        let purlin_s = project.geometry.purlin_spacing as f32;
        let half_slope_len = ((b * 0.5).powi(2) + (h_ridge - h_eave).powi(2)).sqrt();
        let num_purlins = (half_slope_len / purlin_s).round() as usize;

        for p in 1..num_purlins {
            let t = p as f32 / num_purlins as f32;
            let px_l = t * (b * 0.5);
            let py_l = h_eave + t * (h_ridge - h_eave);
            draw_line_3d(
                painter,
                camera,
                screen,
                Vec3::new(px_l, py_l, 0.0),
                Vec3::new(px_l, py_l, a),
                purlin_stroke,
            );

            let px_r = b - px_l;
            draw_line_3d(
                painter,
                camera,
                screen,
                Vec3::new(px_r, py_l, 0.0),
                Vec3::new(px_r, py_l, a),
                purlin_stroke,
            );
        }
    }

    // Destaque da Abertura Dominante (Portão aberto em barlavento)
    if project.openings.dominant_area > 0.0 {
        let gate_w = 4.0f32;
        let gate_h = (project.openings.dominant_area as f32 / gate_w).min(h_eave * 0.85);
        let gate_z = a * 0.5;
        let gate_stroke = Stroke::new(2.5, Color32::from_rgb(255, 87, 34));

        draw_line_3d(
            painter,
            camera,
            screen,
            Vec3::new(0.0, 0.0, gate_z - gate_w * 0.5),
            Vec3::new(0.0, gate_h, gate_z - gate_w * 0.5),
            gate_stroke,
        );
        draw_line_3d(
            painter,
            camera,
            screen,
            Vec3::new(0.0, 0.0, gate_z + gate_w * 0.5),
            Vec3::new(0.0, gate_h, gate_z + gate_w * 0.5),
            gate_stroke,
        );
        draw_line_3d(
            painter,
            camera,
            screen,
            Vec3::new(0.0, gate_h, gate_z - gate_w * 0.5),
            Vec3::new(0.0, gate_h, gate_z + gate_w * 0.5),
            gate_stroke,
        );
    }

    // Flechas Vetoriais de Carga e Pressão Dinâmica em Tempo Real
    if state.show_vectors {
        let v_scale = state.vector_scale * 1.5;
        let wind_color = EoloTheme::wind_gold(dark);
        let press_color = EoloTheme::overpressure_red(dark);
        let suct_color = EoloTheme::suction_cyan(dark);
        let eave_color = EoloTheme::eave_magenta(dark);

        // 1. Seta Grande de Direção do Vento Incidente
        let ang_rad = state.wind_incidence_angle_deg.to_radians();
        let wind_dir = Vec3::new(-ang_rad.sin(), 0.0, -ang_rad.cos());
        let wind_origin = Vec3::new(
            b * 0.5 - wind_dir.x * (b * 0.8 + 6.0),
            h_eave * 0.5,
            a * 0.5 - wind_dir.z * (a * 0.8 + 6.0),
        );
        draw_arrow_3d(
            painter,
            camera,
            screen,
            wind_origin,
            wind_dir,
            12.0 * v_scale,
            wind_color,
            4.0,
        );

        // 2. Flechas de Sobrepressão na Parede de Barlavento
        let num_arrows_z = 5;
        for iz in 1..num_arrows_z {
            let az = iz as f32 * (a / num_arrows_z as f32);
            for iy in [h_eave * 0.35, h_eave * 0.75] {
                let orig = Vec3::new(-1.0, iy, az);
                let dir_inward = Vec3::new(1.0, 0.0, 0.0);
                let len = (env.wall_windward_kn_m2.abs() as f32 * 2.5 * v_scale).clamp(2.0, 8.0);
                draw_arrow_3d(painter, camera, screen, orig, dir_inward, len, press_color, 2.0);
            }
        }

        // 3. Flechas de Sucção / Arrancamento no Telhado
        let roof_uplift_len =
            (env.roof_max_uplift_kn_m2.abs() as f32 * 2.2 * v_scale).clamp(2.5, 9.0);
        for iz in 1..num_arrows_z {
            let az = iz as f32 * (a / num_arrows_z as f32);

            let p_roof_l = Vec3::new(b * 0.25, h_eave + (h_ridge - h_eave) * 0.5, az);
            let normal_l = Vec3::new(-0.3, 0.95, 0.0).normalize();
            draw_arrow_3d(
                painter,
                camera,
                screen,
                p_roof_l,
                normal_l,
                roof_uplift_len,
                suct_color,
                2.0,
            );

            let p_roof_r = Vec3::new(b * 0.75, h_eave + (h_ridge - h_eave) * 0.5, az);
            let normal_r = Vec3::new(0.3, 0.95, 0.0).normalize();
            let roof_leeward_len =
                (env.roof_leeward_kn_m2.abs() as f32 * 2.0 * v_scale).clamp(2.0, 7.0);
            draw_arrow_3d(
                painter,
                camera,
                screen,
                p_roof_r,
                normal_r,
                roof_leeward_len,
                suct_color,
                1.8,
            );
        }

        // 4. Flechas de Arrancamento no Beiral
        let eave_uplift_len =
            (env.eave_max_uplift_kn_m2.abs() as f32 * 2.8 * v_scale).clamp(3.0, 10.0);
        for iz in 1..num_arrows_z {
            let az = iz as f32 * (a / num_arrows_z as f32);
            let p_eave = Vec3::new(-eave_x, h_eave, az);
            let dir_up = Vec3::new(0.0, 1.0, 0.0);
            draw_arrow_3d(
                painter,
                camera,
                screen,
                p_eave,
                dir_up,
                eave_uplift_len,
                eave_color,
                2.2,
            );
        }
    }
}
