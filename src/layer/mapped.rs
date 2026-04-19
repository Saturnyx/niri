use std::sync::Arc;

use niri_config::utils::MergeWith as _;
use niri_config::{Config, CornerRadius, LayerRule};
use smithay::backend::renderer::element::surface::WaylandSurfaceRenderElement;
use smithay::backend::renderer::element::Kind;
<<<<<<< HEAD
use smithay::backend::renderer::utils::RendererSurfaceStateUserData;
use smithay::desktop::{LayerSurface, PopupManager};
use smithay::utils::{Logical, Point, Rectangle, Scale, Size};
use smithay::wayland::compositor::with_states;
=======
use smithay::desktop::{LayerSurface, PopupKind, PopupManager};
use smithay::utils::{Logical, Point, Rectangle, Scale, Size};
use smithay::wayland::compositor::{remove_pre_commit_hook, HookId};
>>>>>>> upstream/main
use smithay::wayland::shell::wlr_layer::{ExclusiveZone, Layer};

use super::ResolvedLayerRules;
use crate::animation::Clock;
use crate::handlers::background_effect::get_cached_blur_region;
use crate::layout::shadow::Shadow;
use crate::niri_render_elements;
<<<<<<< HEAD
use crate::render_helpers::background_effect::{BackgroundEffect, BackgroundEffectElement};
=======
use crate::render_helpers::background_effect::BackgroundEffectElement;
>>>>>>> upstream/main
use crate::render_helpers::renderer::NiriRenderer;
use crate::render_helpers::shadow::ShadowRenderElement;
use crate::render_helpers::solid_color::{SolidColorBuffer, SolidColorRenderElement};
use crate::render_helpers::surface::push_elements_from_surface_tree;
<<<<<<< HEAD
=======
use crate::render_helpers::xray::XrayPos;
>>>>>>> upstream/main
use crate::render_helpers::{background_effect, RenderCtx};
use crate::utils::{baba_is_float_offset, round_logical_in_physical};

#[derive(Debug)]
pub struct MappedLayer {
    /// The surface itself.
    surface: LayerSurface,

    /// Pre-commit hook that we have on all mapped layer surfaces.
    pre_commit_hook: HookId,

    /// Up-to-date rules.
    rules: ResolvedLayerRules,

    /// Whether to recompute layer rules on the next commit.
    ///
    /// Set in the pre-commit hook when the layer changes; consumed in the commit handler.
    recompute_rules_on_commit: bool,

    /// Buffer to draw instead of the surface when it should be blocked out.
    block_out_buffer: SolidColorBuffer,

    /// The shadow around the surface.
    shadow: Shadow,

<<<<<<< HEAD
    /// The background effect, like blur, behind the layer-surface.
    background_effect: BackgroundEffect,
=======
    /// The blur config, passed for background effect rendering.
    blur_config: niri_config::Blur,
>>>>>>> upstream/main

    /// The view size for the layer surface's output.
    view_size: Size<f64, Logical>,

    /// Scale of the output the layer surface is on (and rounds its sizes to).
    scale: f64,

    /// Clock for driving animations.
    clock: Clock,
}

niri_render_elements! {
    LayerSurfaceRenderElement<R> => {
        Wayland = WaylandSurfaceRenderElement<R>,
        SolidColor = SolidColorRenderElement,
        Shadow = ShadowRenderElement,
        BackgroundEffect = BackgroundEffectElement,
    }
}

impl MappedLayer {
    pub fn new(
        surface: LayerSurface,
        pre_commit_hook: HookId,
        rules: ResolvedLayerRules,
        view_size: Size<f64, Logical>,
        scale: f64,
        clock: Clock,
        config: &Config,
    ) -> Self {
        let mut shadow_config = config.layout.shadow;
        // Shadows for layer surfaces need to be explicitly enabled.
        shadow_config.on = false;
        shadow_config.merge_with(&rules.shadow);

        Self {
            surface,
            pre_commit_hook,
            rules,
            recompute_rules_on_commit: false,
            block_out_buffer: SolidColorBuffer::new((0., 0.), [0., 0., 0., 1.]),
            view_size,
            scale,
            shadow: Shadow::new(shadow_config),
<<<<<<< HEAD
            background_effect: BackgroundEffect::new(),
=======
            blur_config: config.blur,
>>>>>>> upstream/main
            clock,
        }
    }

    pub fn update_config(&mut self, config: &Config) {
        let mut shadow_config = config.layout.shadow;
        // Shadows for layer surfaces need to be explicitly enabled.
        shadow_config.on = false;
        shadow_config.merge_with(&self.rules.shadow);
        self.shadow.update_config(shadow_config);

<<<<<<< HEAD
        self.background_effect.update_config(config.blur);
=======
        self.blur_config = config.blur;
>>>>>>> upstream/main
    }

    pub fn update_shaders(&mut self) {
        self.shadow.update_shaders();
    }

    pub fn update_sizes(&mut self, view_size: Size<f64, Logical>, scale: f64) {
        self.view_size = view_size;
        self.scale = scale;
    }

    pub fn update_render_elements(&mut self, size: Size<f64, Logical>) {
        // Round to physical pixels.
        let size = size
            .to_physical_precise_round(self.scale)
            .to_logical(self.scale);

        self.block_out_buffer.resize(size);

        let radius = self.rules.geometry_corner_radius.unwrap_or_default();
        // FIXME: is_active based on keyboard focus?
        self.shadow
            .update_render_elements(size, true, radius, self.scale, 1.);

        let has_blur_region = self.blur_region().is_some_and(|r| !r.is_empty());
        self.background_effect.update_render_elements(
            radius,
            self.rules.background_effect,
            has_blur_region,
        );
    }

    pub fn are_animations_ongoing(&self) -> bool {
        self.rules.baba_is_float
    }

    pub fn surface(&self) -> &LayerSurface {
        &self.surface
    }

    pub fn rules(&self) -> &ResolvedLayerRules {
        &self.rules
    }

    /// Recomputes the resolved layer rules and returns whether they changed.
    pub fn recompute_layer_rules(&mut self, rules: &[LayerRule], is_at_startup: bool) -> bool {
        let new_rules = ResolvedLayerRules::compute(rules, &self.surface, is_at_startup);
        if new_rules == self.rules {
            return false;
        }

        self.rules = new_rules;
        true
    }

    pub fn set_recompute_rules_on_commit(&mut self) {
        self.recompute_rules_on_commit = true;
    }

    pub fn take_recompute_rules_on_commit(&mut self) -> bool {
        std::mem::take(&mut self.recompute_rules_on_commit)
    }

    pub fn place_within_backdrop(&self) -> bool {
        if !self.rules.place_within_backdrop {
            return false;
        }

        if self.surface.layer() != Layer::Background {
            return false;
        }

        let state = self.surface.cached_state();
        if state.exclusive_zone != ExclusiveZone::DontCare {
            return false;
        }

        true
    }

    pub fn bob_offset(&self) -> Point<f64, Logical> {
        if !self.rules.baba_is_float {
            return Point::from((0., 0.));
        }

        let y = baba_is_float_offset(self.clock.now(), self.view_size.h);
        let y = round_logical_in_physical(self.scale, y);
        Point::from((0., y))
    }

    pub fn render_normal<R: NiriRenderer>(
        &self,
        mut ctx: RenderCtx<R>,
<<<<<<< HEAD
        location: Point<f64, Logical>,
        mut pos_in_backdrop: Point<f64, Logical>,
        zoom: f64,
=======
        ns: Option<usize>,
        location: Point<f64, Logical>,
        xray_pos: XrayPos,
>>>>>>> upstream/main
        push: &mut dyn FnMut(LayerSurfaceRenderElement<R>),
    ) {
        let scale = Scale::from(self.scale);
        let alpha = self.rules.opacity.unwrap_or(1.).clamp(0., 1.);
<<<<<<< HEAD
        let location = location + self.bob_offset();
        pos_in_backdrop += self.bob_offset().upscale(zoom);

        if ctx.target.should_block_out(self.rules.block_out_from) {
=======

        let bob_offset = self.bob_offset();
        let location = location + bob_offset;
        let xray_pos = xray_pos.offset(bob_offset);

        let surface = self.surface.wl_surface();

        let should_block_out = ctx.target.should_block_out(self.rules.block_out_from);
        if should_block_out {
>>>>>>> upstream/main
            // Round to physical pixels.
            let location = location.to_physical_precise_round(scale).to_logical(scale);

            // FIXME: take geometry-corner-radius into account.
            let elem = SolidColorRenderElement::from_buffer(
                &self.block_out_buffer,
                location,
                alpha,
                Kind::Unspecified,
            );
            push(elem.into());
        } else {
            // Layer surfaces don't have extra geometry like windows.
            let buf_pos = location;

            push_elements_from_surface_tree(
                ctx.renderer,
                surface,
                buf_pos.to_physical_precise_round(scale),
                scale,
                alpha,
                Kind::ScanoutCandidate,
                &mut |elem| push(elem.into()),
            );
        }

        let location = location.to_physical_precise_round(scale).to_logical(scale);
        self.shadow
            .render(ctx.renderer, location, &mut |elem| push(elem.into()));

<<<<<<< HEAD
        if self.background_effect.is_visible() {
            let area = Rectangle::new(location, self.block_out_buffer.size());
            // Effects not requested by the surface itself are drawn to match the geometry.
            let mut clip = true;

            // FIXME: support blur regions on subsurfaces in addition to the main surface.
            let mut subregion = None;
            let blur_geometry = if let Some(rects) = self.blur_region() {
                if rects.is_empty() {
                    // Surface has a set, but empty blur region.
                    None
                } else {
                    // If the surface itself requests the effects, apply different defaults.
                    clip = false;

                    // Use geometry-shaped blur for blocked-out layers to avoid unintentionally
                    // leaking any surface shapes. We render those layers as geometry-shaped solid
                    // rectangles anyway.
                    if ctx.target.should_block_out(self.rules.block_out_from) {
                        clip = true;
                        Some(area)
                    } else {
                        let mut main_surface_geo = self.main_surface_geo().to_f64();
                        main_surface_geo.loc += area.loc;

                        subregion = Some(background_effect::EffectSubregion {
                            rects,
                            scale: Scale::from(1.),
                            offset: main_surface_geo.loc,
                        });

                        main_surface_geo = main_surface_geo
                            .to_physical_precise_round(self.scale)
                            .to_logical(self.scale);
                        Some(main_surface_geo)
                    }
                }
            } else {
                Some(area)
            };

            if let Some(geometry) = blur_geometry {
                pos_in_backdrop += (geometry.loc - area.loc).upscale(zoom);
                let params = background_effect::RenderParams {
                    geometry,
                    subregion,
                    clip: clip.then_some((area, CornerRadius::default())),
                    pos_in_backdrop,
                    zoom,
                    scale: self.scale,
                };
                self.background_effect
                    .render(ctx.as_gles(), params, &mut |elem| push(elem.into()));
            }
        }
=======
        let geometry = Rectangle::new(location, self.block_out_buffer.size());
        let surface_off = Point::new(0., 0.); // No geometry on layer surfaces.
        let surface_anim_scale = Scale::from(1.);
        let radius = self.rules.geometry_corner_radius.unwrap_or_default();
        background_effect::render_for_tile(
            ctx.as_gles(),
            ns,
            geometry,
            self.scale,
            false,
            surface,
            surface_off,
            surface_anim_scale,
            self.blur_config,
            radius,
            self.rules.background_effect,
            should_block_out,
            xray_pos,
            &mut |elem| push(elem.into()),
        );
>>>>>>> upstream/main
    }

    pub fn render_popups<R: NiriRenderer>(
        &self,
<<<<<<< HEAD
        ctx: RenderCtx<R>,
        location: Point<f64, Logical>,
        push: &mut dyn FnMut(LayerSurfaceRenderElement<R>),
    ) {
        let scale = Scale::from(self.scale);
        let alpha = self.rules.opacity.unwrap_or(1.).clamp(0., 1.);
        let location = location + self.bob_offset();

=======
        mut ctx: RenderCtx<R>,
        ns: Option<usize>,
        location: Point<f64, Logical>,
        xray_pos: XrayPos,
        push: &mut dyn FnMut(LayerSurfaceRenderElement<R>),
    ) {
>>>>>>> upstream/main
        if ctx.target.should_block_out(self.rules.block_out_from) {
            return;
        }

        let scale = Scale::from(self.scale);
        let alpha = self.rules.opacity.unwrap_or(1.).clamp(0., 1.);

        let bob_offset = self.bob_offset();
        let location = location + bob_offset;
        let xray_pos = xray_pos.offset(bob_offset);

        let surface = self.surface.wl_surface();
        for (popup, offset) in PopupManager::popups_for_surface(surface) {
            let popup_rules = match popup {
                PopupKind::Xdg(_) => self.rules.popups,
                // IME popups aren't affected by rules for regular popups.
                PopupKind::InputMethod(_) => niri_config::ResolvedPopupsRules::default(),
            };
            let alpha = alpha * popup_rules.opacity.unwrap_or(1.).clamp(0., 1.);

            let surface = popup.wl_surface();
            let popup_geo = popup.geometry();
            let surface_loc = location + (offset - popup_geo.loc).to_f64();

            push_elements_from_surface_tree(
                ctx.renderer,
<<<<<<< HEAD
                popup.wl_surface(),
                (buf_pos + offset.to_f64()).to_physical_precise_round(scale),
=======
                surface,
                surface_loc.to_physical_precise_round(scale),
>>>>>>> upstream/main
                scale,
                alpha,
                Kind::ScanoutCandidate,
                &mut |elem| push(elem.into()),
            );

            let geometry = Rectangle::new(location + offset.to_f64(), popup_geo.size.to_f64());
            let surface_off = popup_geo.loc.upscale(-1).to_f64();
            let surface_anim_scale = Scale::from(1.);
            let mut effect = popup_rules.background_effect;
            // Default xray to false for pop-ups since they're always on top of something.
            if effect.xray.is_none() {
                effect.xray = Some(false);
            }
            let xray_pos = xray_pos.offset(offset.to_f64());
            background_effect::render_for_tile(
                ctx.as_gles(),
                ns,
                geometry,
                self.scale,
                false,
                surface,
                surface_off,
                surface_anim_scale,
                self.blur_config,
                popup_rules.geometry_corner_radius.unwrap_or_default(),
                effect,
                false,
                xray_pos,
                &mut |elem| push(elem.into()),
            );
        }
    }

    fn main_surface_geo(&self) -> Rectangle<i32, Logical> {
        with_states(self.surface.wl_surface(), |states| {
            let data = states.data_map.get::<RendererSurfaceStateUserData>();
            data.and_then(|d| d.lock().unwrap().view())
                .map(|view| Rectangle {
                    loc: view.offset,
                    size: view.dst,
                })
        })
        .unwrap_or_default()
    }

    fn blur_region(&self) -> Option<Arc<Vec<Rectangle<i32, Logical>>>> {
        with_states(self.surface.wl_surface(), get_cached_blur_region)
    }
}

impl Drop for MappedLayer {
    fn drop(&mut self) {
        remove_pre_commit_hook(self.surface.wl_surface(), &self.pre_commit_hook);
    }
}
