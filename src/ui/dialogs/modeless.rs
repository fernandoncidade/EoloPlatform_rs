use egui::{Context, Id, LayerId, Order, Rect, Ui, Vec2, ViewportClass, Window};

use crate::ui::theme::EoloTheme;

/// Estado de controle de Janela Modeless (Não-Modal) com gerenciamento de Z-Order (Profundidade / Sobreposição).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ModelessState {
    pub is_open: bool,
    pub raise_z_order: bool,
    pub maximized: bool,
    pub minimized: bool,
}

impl ModelessState {
    /// Abre a janela secundária modeless ou, se já estiver aberta, eleva seu Z-Order para o topo (raise).
    pub fn open(&mut self) {
        self.is_open = true;
        self.minimized = false;
        self.raise_z_order = true;
    }

    /// Fecha a janela secundária modeless.
    pub fn close(&mut self) {
        self.is_open = false;
        self.minimized = false;
        self.raise_z_order = false;
    }
}

/// Obtém o retângulo de tela seguro a partir do contexto egui.
pub fn screen_rect(ctx: &Context) -> Rect {
    ctx.input(|i| i.raw.screen_rect)
        .unwrap_or(Rect::from_min_size(
            egui::Pos2::ZERO,
            Vec2::new(1280.0, 800.0),
        ))
}

/// Renderiza uma Janela Secundária / Diálogo Modeless Nativo do Windows 11 com suporte completo a Z-Order.
///
/// - **GUI Principal como Janela "Pai" (`ViewportId::ROOT`):** A janela secundária abre centralizada sobre a
///   janela Pai no mesmo monitor e é encerrada automaticamente caso a janela Pai seja fechada.
/// - **Modeless (Não-Modal):** Permite alternar livremente o foco e a interação entre a GUI principal Pai e
///   todas as janelas secundárias abertas sem travar a interface principal.
/// - **Z-Order (Ordem no Eixo Z / Raise):** Ao clicar em qualquer janela aberta ou acionar seu botão na barra
///   de status / menu, ela é elevada imediatamente para o topo da pilha visual (frente de todas as demais).
/// - **Dimensões Controladas:** Respeita estritamente `default_size` e `min_size`, impedindo auto-expansão no eixo Y.
#[allow(clippy::too_many_arguments)]
pub fn show_modeless_dialog<R>(
    ctx: &Context,
    viewport_id_str: &str,
    title: impl Into<String>,
    default_size: [f32; 2],
    min_size: [f32; 2],
    dark_mode: bool,
    state: &mut ModelessState,
    mut content: impl FnMut(&mut Ui, &mut bool) -> R,
) -> Option<R> {
    if !state.is_open {
        return None;
    }

    // Se a janela Pai (ROOT) estiver sendo fechada, encerra as janelas filhas
    let root_closing = ctx.input_for(egui::ViewportId::ROOT, |i| i.viewport().close_requested());
    if root_closing {
        state.close();
        return None;
    }

    let viewport_id = egui::ViewportId::from_hash_of(viewport_id_str);
    let title_str = title.into();

    // Obtém a posição da GUI principal (Pai) para centralizar a janela secundária ao abrir
    let root_rect = ctx.input_for(egui::ViewportId::ROOT, |i| {
        i.viewport().outer_rect.or(i.viewport().inner_rect)
    });

    let target_pos = if let Some(root) = root_rect {
        let cx = root.center().x;
        let cy = root.center().y;
        let mut x = cx - default_size[0] * 0.5;
        let mut y = cy - default_size[1] * 0.5;
        if y < root.min.y + 24.0 {
            y = root.min.y + 24.0;
        }
        if x < root.min.x + 24.0 {
            x = root.min.x + 24.0;
        }
        egui::pos2(x, y)
    } else {
        let srect = screen_rect(ctx);
        egui::pos2(
            (srect.center().x - default_size[0] * 0.5).max(20.0),
            (srect.center().y - default_size[1] * 0.5).max(20.0),
        )
    };

    let viewport_exists = ctx
        .input_for(viewport_id, |i| {
            i.viewport().outer_rect.or(i.viewport().inner_rect)
        })
        .is_some();

    if state.raise_z_order {
        ctx.send_viewport_cmd_to(viewport_id, egui::ViewportCommand::Minimized(false));
        ctx.send_viewport_cmd_to(viewport_id, egui::ViewportCommand::Focus);
        if !viewport_exists {
            ctx.send_viewport_cmd_to(viewport_id, egui::ViewportCommand::OuterPosition(target_pos));
        }
        state.raise_z_order = false;
    }

    let mut builder = egui::ViewportBuilder::default()
        .with_title(title_str.clone())
        .with_inner_size(default_size)
        .with_min_inner_size(min_size)
        .with_decorations(true)
        .with_resizable(true)
        .with_close_button(true)
        .with_minimize_button(true)
        .with_maximize_button(true);

    if !viewport_exists {
        builder = builder.with_position([target_pos.x, target_pos.y]);
    }

    let mut close_requested = false;
    let mut result = None;
    let raise_embedded = state.raise_z_order;

    ctx.show_viewport_immediate(viewport_id, builder, |vp_ui, class| {
        let vp_ctx = vp_ui.ctx().clone();
        // Sincroniza dinamicamente o tema Escuro/Claro com a janela Pai em tempo de execução
        EoloTheme::apply(&vp_ctx, dark_mode);

        if class == ViewportClass::EmbeddedWindow {
            let win_id = Id::from(viewport_id);
            let mut open = state.is_open;
            let resp = Window::new(title_str.clone())
                .id(win_id)
                .order(Order::Middle)
                .default_size(default_size)
                .min_size(min_size)
                .max_size(default_size)
                .resizable(true)
                .collapsible(false)
                .open(&mut open)
                .show(vp_ui, |ui| {
                    *ui.style_mut() = (*vp_ctx.global_style()).clone();
                    result = Some(content(ui, &mut close_requested));
                });

            if let Some(r) = resp {
                if r.response.clicked() || r.response.dragged() || raise_embedded {
                    vp_ctx.move_to_top(LayerId::new(Order::Middle, win_id));
                }
            }
            if !open {
                close_requested = true;
            }
        } else {
            if vp_ctx.input(|i| i.viewport().close_requested()) {
                close_requested = true;
            }

            egui::CentralPanel::default().show(vp_ui, |ui| {
                *ui.style_mut() = (*vp_ctx.global_style()).clone();
                result = Some(content(ui, &mut close_requested));
            });
        }
    });

    if close_requested {
        state.close();
    }

    result
}
