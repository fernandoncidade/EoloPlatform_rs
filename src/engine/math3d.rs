use egui::{Color32, Pos2, Rect, Stroke};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub const ZERO: Self = Self { x: 0.0, y: 0.0, z: 0.0 };

    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn add(self, other: Self) -> Self {
        Self::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }

    #[allow(clippy::should_implement_trait)]
    pub fn sub(self, other: Self) -> Self {
        Self::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }

    pub fn scale(self, s: f32) -> Self {
        Self::new(self.x * s, self.y * s, self.z * s)
    }

    pub fn dot(self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(self, other: Self) -> Self {
        Self::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    pub fn length(self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn normalize(self) -> Self {
        let l = self.length();
        if l > 1e-6 {
            self.scale(1.0 / l)
        } else {
            Self::ZERO
        }
    }
}

/// Câmera Orbital 3D interativa otimizada para o egui::Painter
#[derive(Debug, Clone)]
pub struct OrbitCamera {
    /// Ângulo azimutal de rotação horizontal (graus)
    pub azimuth_deg: f32,
    /// Ângulo de elevação vertical (graus, -85° a +85°)
    pub elevation_deg: f32,
    /// Distância da câmera ao centro do modelo (zoom)
    pub distance: f32,
    /// Centro de foco / rotação da câmera (ponto no galpão)
    pub target: Vec3,
    /// Campo de visão vertical em radianos
    pub fov_rad: f32,
    /// Projeção em perspectiva ou ortográfica
    pub perspective: bool,
}

impl Default for OrbitCamera {
    fn default() -> Self {
        Self {
            azimuth_deg: 45.0,
            elevation_deg: 25.0,
            distance: 65.0,
            target: Vec3::new(10.0, 4.0, 20.0), // Centro típico de galpão 20x40m
            fov_rad: 50.0f32.to_radians(),
            perspective: true,
        }
    }
}

impl OrbitCamera {
    /// Posição no espaço mundial (World Space) calculada a partir dos ângulos esféricos
    pub fn eye_position(&self) -> Vec3 {
        let az_rad = self.azimuth_deg.to_radians();
        let el_rad = self.elevation_deg.to_radians();

        let cos_el = el_rad.cos();
        let sin_el = el_rad.sin();
        let sin_az = az_rad.sin();
        let cos_az = az_rad.cos();

        let dx = self.distance * cos_el * sin_az;
        let dy = self.distance * sin_el;
        let dz = self.distance * cos_el * cos_az;

        Vec3::new(self.target.x + dx, self.target.y + dy, self.target.z + dz)
    }

    /// Transforma um ponto do mundo para coordenadas da tela (Screen Space Pos2)
    /// Retorna Some((pos2, depth)) se estiver na frente da câmera, ou None se estiver atrás
    pub fn project(&self, point: Vec3, screen: Rect) -> Option<(Pos2, f32)> {
        let eye = self.eye_position();
        let forward = self.target.sub(eye).normalize();
        let world_up = Vec3::new(0.0, 1.0, 0.0);
        let right = forward.cross(world_up).normalize();
        let up = right.cross(forward).normalize();

        // Vetor da câmera até o ponto
        let rel = point.sub(eye);
        let depth = rel.dot(forward);

        if self.perspective && depth <= 0.1 {
            return None; // Atrás da câmera
        }

        let x_cam = rel.dot(right);
        let y_cam = rel.dot(up);

        let (norm_x, norm_y) = if self.perspective {
            let scale = 1.0 / (depth * (self.fov_rad * 0.5).tan());
            (x_cam * scale, y_cam * scale)
        } else {
            let scale = 2.0 / self.distance;
            (x_cam * scale, y_cam * scale)
        };

        let aspect = screen.width() / screen.height().max(1.0);
        let half_w = screen.width() * 0.5;
        let half_h = screen.height() * 0.5;

        let screen_x = screen.center().x + (norm_x / aspect) * half_w;
        // Inversão do eixo Y da tela (Y no egui cresce para baixo)
        let screen_y = screen.center().y - norm_y * half_h;

        Some((Pos2::new(screen_x, screen_y), depth))
    }

    /// Processa eventos de arraste do mouse para orbitar, transladar (pan) e dar zoom
    pub fn handle_input(&mut self, ui: &egui::Ui, response: &egui::Response) {
        if response.dragged_by(egui::PointerButton::Primary) {
            let delta = response.drag_delta();
            self.azimuth_deg = (self.azimuth_deg - delta.x * 0.5).rem_euclid(360.0);
            self.elevation_deg = (self.elevation_deg + delta.y * 0.5).clamp(-85.0, 85.0);
        } else if response.dragged_by(egui::PointerButton::Secondary)
            || response.dragged_by(egui::PointerButton::Middle)
        {
            let delta = response.drag_delta();
            let az_rad = self.azimuth_deg.to_radians();
            let sin_az = az_rad.sin();
            let cos_az = az_rad.cos();

            let pan_speed = self.distance * 0.0015;
            let right = Vec3::new(cos_az, 0.0, -sin_az);
            let up = Vec3::new(0.0, 1.0, 0.0);

            self.target = self
                .target
                .sub(right.scale(delta.x * pan_speed))
                .add(up.scale(delta.y * pan_speed));
        }

        let scroll_delta = ui.input(|i| i.smooth_scroll_delta.y);
        if scroll_delta != 0.0 {
            let factor = if scroll_delta > 0.0 { 0.90 } else { 1.10 };
            self.distance = (self.distance * factor).clamp(5.0, 300.0);
        }
    }

    /// Reseta para a vista isométrica padrão
    pub fn reset_view(&mut self, center: Vec3, span: f32) {
        self.azimuth_deg = 45.0;
        self.elevation_deg = 25.0;
        self.target = center;
        self.distance = (span * 2.2).clamp(25.0, 180.0);
    }
}

/// Desenha uma linha 3D no canvas do egui
pub fn draw_line_3d(
    painter: &egui::Painter,
    camera: &OrbitCamera,
    screen: Rect,
    p1: Vec3,
    p2: Vec3,
    stroke: Stroke,
) {
    if let (Some((s1, _)), Some((s2, _))) = (camera.project(p1, screen), camera.project(p2, screen)) {
        painter.line_segment([s1, s2], stroke);
    }
}

/// Desenha uma seta 3D vetorial de sobrepressão ou sucção
/// - Sobrepressão: seta apontando para a superfície (inward)
/// - Sucção: seta puxando para longe da superfície (outward)
#[allow(clippy::too_many_arguments)]
pub fn draw_arrow_3d(
    painter: &egui::Painter,
    camera: &OrbitCamera,
    screen: Rect,
    origin: Vec3,
    direction_normalized: Vec3,
    length: f32,
    color: Color32,
    stroke_width: f32,
) {
    if length <= 0.01 {
        return;
    }
    let target = origin.add(direction_normalized.scale(length));

    if let (Some((p_orig, _)), Some((p_dest, _))) =
        (camera.project(origin, screen), camera.project(target, screen))
    {
        let stroke = Stroke::new(stroke_width, color);
        painter.line_segment([p_orig, p_dest], stroke);

        // Ponta da seta 2D em direção a p_dest
        let dir_screen = Vec3::new(p_dest.x - p_orig.x, p_dest.y - p_orig.y, 0.0);
        let len_screen = dir_screen.length();
        if len_screen > 6.0 {
            let u = dir_screen.normalize();
            let perp = Vec3::new(-u.y, u.x, 0.0);
            let head_len = (len_screen * 0.25).clamp(4.0, 12.0);
            let head_w = head_len * 0.55;

            let base = Pos2::new(p_dest.x - u.x * head_len, p_dest.y - u.y * head_len);
            let wing1 = Pos2::new(base.x + perp.x * head_w, base.y + perp.y * head_w);
            let wing2 = Pos2::new(base.x - perp.x * head_w, base.y - perp.y * head_w);

            painter.line_segment([p_dest, wing1], stroke);
            painter.line_segment([p_dest, wing2], stroke);
        }
    }
}
