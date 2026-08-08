//! Cleanroom Rust port of upstream Go source file: `layer.go`
//! Upstream Target Tag / Version: `v2.0.5`
//!
//! <public-docs>
//! A visual layer hierarchy with z-ordering, hit testing, and composition.
//! </public-docs>

use std::collections::HashMap;

use crate::canvas::Drawable;
use crate::size;

/// A simple rectangle.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rectangle {
    /// The minimum corner (x, y).
    pub min: (usize, usize),
    /// The maximum corner (x, y).
    pub max: (usize, usize),
}

impl Default for Rectangle {
    fn default() -> Self {
        Rectangle {
            min: (0, 0),
            max: (0, 0),
        }
    }
}

impl Rectangle {
    /// Returns the width of the rectangle.
    pub fn dx(&self) -> usize {
        self.max.0.saturating_sub(self.min.0)
    }
    /// Returns the height of the rectangle.
    pub fn dy(&self) -> usize {
        self.max.1.saturating_sub(self.min.1)
    }
    /// Returns the union of this rectangle with another.
    pub fn union(&self, other: Rectangle) -> Rectangle {
        Rectangle {
            min: (self.min.0.min(other.min.0), self.min.1.min(other.min.1)),
            max: (self.max.0.max(other.max.0), self.max.1.max(other.max.1)),
        }
    }
    /// Returns whether the point is inside the rectangle.
    pub fn contains(&self, x: usize, y: usize) -> bool {
        x >= self.min.0 && x < self.max.0 && y >= self.min.1 && y < self.max.1
    }
    /// Returns whether this rectangle overlaps another.
    pub fn overlaps(&self, other: &Rectangle) -> bool {
        self.min.0 < other.max.0
            && other.min.0 < self.max.0
            && self.min.1 < other.max.1
            && other.min.1 < self.max.1
    }
}

/// <upstream-comment>Layer represents a visual layer with content and positioning. It's a pure
/// data structure that defines the layer hierarchy without any computation.</upstream-comment>
#[derive(Debug, Clone)]
pub struct Layer {
    id: String,
    content: String,
    width: usize,
    height: usize,
    x: isize,
    y: isize,
    z: isize,
    layers: Vec<Layer>,
}

/// <upstream-comment>NewLayer creates a new [Layer] with the given content and optional child
/// layers.</upstream-comment>
pub fn new_layer(content: &str, layers: &[Layer]) -> Layer {
    let mut l = Layer {
        id: String::new(),
        content: content.to_string(),
        width: 0,
        height: 0,
        x: 0,
        y: 0,
        z: 0,
        layers: Vec::new(),
    };
    l.add_layers(layers);
    l
}

impl Layer {
    /// Returns a new Layer with the given content.
    pub fn new(content: &str) -> Layer {
        new_layer(content, &[])
    }

    /// <upstream-comment>GetContent returns the content of the Layer.</upstream-comment>
    pub fn get_content(&self) -> &str {
        &self.content
    }

    /// <upstream-comment>Width returns the width of the Layer.</upstream-comment>
    pub fn width(&self) -> usize {
        self.width
    }

    /// <upstream-comment>Height returns the height of the Layer.</upstream-comment>
    pub fn height(&self) -> usize {
        self.height
    }

    /// <upstream-comment>GetID returns the ID of the Layer.</upstream-comment>
    pub fn get_id(&self) -> &str {
        &self.id
    }

    /// <upstream-comment>ID sets the ID of the Layer.</upstream-comment>
    pub fn id(mut self, id: &str) -> Layer {
        self.id = id.to_string();
        self
    }

    /// <upstream-comment>X sets the x-coordinate of the Layer relative to its parent.</upstream-comment>
    pub fn x(mut self, x: isize) -> Layer {
        self.x = x;
        self
    }

    /// <upstream-comment>Y sets the y-coordinate of the Layer relative to its parent.</upstream-comment>
    pub fn y(mut self, y: isize) -> Layer {
        self.y = y;
        self
    }

    /// <upstream-comment>Z sets the z-index of the Layer relative to its parent.</upstream-comment>
    pub fn z(mut self, z: isize) -> Layer {
        self.z = z;
        self
    }

    /// <upstream-comment>GetX returns the x-coordinate of the Layer relative to its parent.</upstream-comment>
    pub fn get_x(&self) -> isize {
        self.x
    }

    /// <upstream-comment>GetY returns the y-coordinate of the Layer relative to its parent.</upstream-comment>
    pub fn get_y(&self) -> isize {
        self.y
    }

    /// <upstream-comment>GetZ returns the z-index of the Layer relative to its parent.</upstream-comment>
    pub fn get_z(&self) -> isize {
        self.z
    }

    /// <upstream-comment>AddLayers adds child layers to the Layer.</upstream-comment>
    pub fn add_layers(&mut self, layers: &[Layer]) -> &mut Layer {
        self.layers.extend(layers.iter().cloned());
        let area = self.bounds_with_offset(0, 0);
        self.width = area.dx();
        self.height = area.dy();
        self
    }

    /// <upstream-comment>GetLayer returns a descendant layer by its ID, or nil if not found.
    /// Layers with empty IDs are skipped.</upstream-comment>
    pub fn get_layer(&self, id: &str) -> Option<&Layer> {
        if id.is_empty() {
            return None;
        }
        if self.id == id {
            return Some(self);
        }
        for child in &self.layers {
            if let Some(found) = child.get_layer(id) {
                return Some(found);
            }
        }
        None
    }

    /// <upstream-comment>MaxZ returns the maximum z-index among this layer and all its descendants.</upstream-comment>
    pub fn max_z(&self) -> isize {
        let mut max_z = self.z;
        for child in &self.layers {
            max_z = max_z.max(child.max_z());
        }
        max_z
    }

    /// boundsWithOffset calculates bounds with parent offset applied.
    pub(crate) fn bounds_with_offset(&self, parent_x: isize, parent_y: isize) -> Rectangle {
        let abs_x = self.x + parent_x;
        let abs_y = self.y + parent_y;

        let (width, height) = (size::width(&self.content), size::height(&self.content));
        let mut bounds = Rectangle {
            min: (abs_x.max(0) as usize, abs_y.max(0) as usize),
            max: (
                (abs_x + width as isize).max(0) as usize,
                (abs_y + height as isize).max(0) as usize,
            ),
        };

        for child in &self.layers {
            bounds = bounds.union(child.bounds_with_offset(abs_x, abs_y));
        }

        bounds
    }
}

/// LayerHit represents the result of a hit test on a [Layer].
#[derive(Debug, Clone, Default)]
pub struct LayerHit {
    id: String,
    layer: Option<Layer>,
    bounds: Rectangle,
}

impl LayerHit {
    /// Empty returns true if the LayerHit represents no hit.
    pub fn empty(&self) -> bool {
        self.layer.is_none()
    }

    /// ID returns the ID of the hit Layer.
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Layer returns the layer that was hit.
    pub fn layer(&self) -> Option<&Layer> {
        self.layer.as_ref()
    }

    /// Bounds returns the bounds of the LayerHit.
    pub fn bounds(&self) -> Rectangle {
        self.bounds
    }
}

/// A flattened layer with its calculated absolute position and bounds.
#[derive(Debug, Clone)]
struct CompositeLayer {
    layer: Layer,
    bounds: Rectangle,
}

fn flatten_recursive(
    layers: &mut Vec<CompositeLayer>,
    index: &mut HashMap<String, Layer>,
    layer: &Layer,
    parent_x: isize,
    parent_y: isize,
) {
    let abs_x = layer.x + parent_x;
    let abs_y = layer.y + parent_y;

    let (width, height) = (size::width(&layer.content), size::height(&layer.content));
    let bounds = Rectangle {
        min: (abs_x.max(0) as usize, abs_y.max(0) as usize),
        max: (
            (abs_x + width as isize).max(0) as usize,
            (abs_y + height as isize).max(0) as usize,
        ),
    };

    layers.push(CompositeLayer {
        layer: layer.clone(),
        bounds,
    });

    // Index layer by ID if it has one.
    if !layer.id.is_empty() {
        index.insert(layer.id.clone(), layer.clone());
    }

    for child in &layer.layers {
        flatten_recursive(layers, index, child, abs_x, abs_y);
    }
}

/// <upstream-comment>Compositor manages the composition of layers. It flattens a layer hierarchy
/// once and provides efficient drawing and hit testing operations. All computation
/// related to layers happens in the Compositor.</upstream-comment>
#[derive(Debug, Clone)]
pub struct Compositor {
    root: Layer,
    layers: Vec<CompositeLayer>,
    index: HashMap<String, Layer>,
    bounds: Rectangle,
}

/// <upstream-comment>NewCompositor creates a new Compositor with an internal root layer. Optional
/// layers can be provided which will be added as children of the root. The layer
/// hierarchy is flattened and sorted by z-index for efficient rendering and hit
/// testing.</upstream-comment>
pub fn new_compositor(layers: &[Layer]) -> Compositor {
    let mut root = Layer::new("");
    root.add_layers(layers);
    let mut c = Compositor {
        root,
        layers: Vec::new(),
        index: HashMap::new(),
        bounds: Rectangle {
            min: (0, 0),
            max: (0, 0),
        },
    };
    c.flatten();
    c
}

impl Compositor {
    /// Returns a new Compositor with the given layers.
    pub fn new(layers: &[Layer]) -> Compositor {
        new_compositor(layers)
    }

    /// <upstream-comment>AddLayers adds layers to the compositor's root and refreshes the internal
    /// state.</upstream-comment>
    pub fn add_layers(&mut self, layers: &[Layer]) -> &mut Compositor {
        self.root.add_layers(layers);
        self.flatten();
        self
    }

    fn flatten(&mut self) {
        self.layers.clear();
        self.index.clear();
        let root = self.root.clone();
        flatten_recursive(&mut self.layers, &mut self.index, &root, 0, 0);

        // Sort by z-index (lowest to highest for drawing).
        self.layers.sort_by_key(|cl| cl.layer.z);

        // Calculate overall bounds.
        if let Some(first) = self.layers.first() {
            let mut bounds = first.bounds;
            for cl in self.layers.iter().skip(1) {
                bounds = bounds.union(cl.bounds);
            }
            self.bounds = bounds;
        }
    }

    /// <upstream-comment>Bounds returns the overall bounds of all layers in the compositor.</upstream-comment>
    pub fn bounds(&self) -> Rectangle {
        self.bounds
    }

    /// <upstream-comment>Hit performs a hit test at the given (x, y) coordinates. If a layer is hit,
    /// it returns the ID of the top-most layer at that point. Layers with empty IDs
    /// are ignored. If no layer is hit, it returns an empty [LayerHit].</upstream-comment>
    pub fn hit(&self, x: usize, y: usize) -> LayerHit {
        // Check from highest z to lowest (reverse order).
        for cl in self.layers.iter().rev() {
            if !cl.layer.id.is_empty() && cl.bounds.contains(x, y) {
                return LayerHit {
                    id: cl.layer.id.clone(),
                    layer: Some(cl.layer.clone()),
                    bounds: cl.bounds,
                };
            }
        }
        LayerHit::default()
    }

    /// <upstream-comment>GetLayer returns a layer by its ID, or nil if not found.
    /// Layers with empty IDs are not indexed and cannot be retrieved.</upstream-comment>
    pub fn get_layer(&self, id: &str) -> Option<&Layer> {
        if id.is_empty() {
            return None;
        }
        self.index.get(id)
    }

    /// <upstream-comment>Refresh re-flattens the layer hierarchy. Call this after modifying the layer
    /// tree structure or positions to update the compositor's internal state.</upstream-comment>
    pub fn refresh(&mut self) {
        self.flatten();
    }

    /// <upstream-comment>Render renders the compositor into a styled string. This is a helper
    /// function that creates a temporary canvas, draws the compositor onto it, and
    /// returns the resulting string.</upstream-comment>
    pub fn render(&self) -> String {
        let (width, height) = (self.bounds.dx(), self.bounds.dy());
        let mut canvas = crate::canvas::Canvas::new(width, height);
        self.draw(&mut canvas, self.bounds);
        canvas.render()
    }
}

impl crate::canvas::Drawable for Compositor {
    fn draw(&self, scr: &mut dyn crate::canvas::Screen, area: crate::layer::Rectangle) {
        for cl in &self.layers {
            if cl.bounds.overlaps(&area) {
                crate::canvas::draw_styled(scr, &cl.layer.content, cl.bounds);
            }
        }
    }
}
