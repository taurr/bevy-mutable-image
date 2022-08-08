use std::ops::Range;

use bevy::{
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};
use image::{GenericImage, GenericImageView, Pixel, Rgba};

#[derive(Component)]
pub struct MutableImageHandle(Handle<Image>);

impl MutableImageHandle {
    pub fn new(width: u32, height: u32, assets: &mut Assets<Image>) -> Self {
        Self(assets.add(Image::new_fill(
            Extent3d {
                width,
                height,
                ..default()
            },
            TextureDimension::D2,
            &[0u8; 4],
            TextureFormat::Rgba8Unorm,
        )))
    }

    #[inline]
    pub fn image_handle(&self) -> Handle<Image> {
        self.into()
    }

    #[inline]
    pub fn image_view<'a>(&'a self, assets: &'a Assets<Image>) -> Option<MutableImageView<'a>> {
        assets.get(&self.0).map(MutableImageView::new)
    }

    #[inline]
    pub fn image<'a>(&self, assets: &'a mut Assets<Image>) -> Option<MutableImage<'a>> {
        assets.get_mut(&self.0).map(MutableImage::new)
    }
}

impl From<&MutableImageHandle> for Handle<Image> {
    #[inline]
    fn from(texture_image: &MutableImageHandle) -> Self {
        texture_image.0.clone()
    }
}

pub struct MutableImageView<'a> {
    data: &'a Vec<u8>,
    width: u32,
    height: u32,
}

impl<'a> MutableImageView<'a> {
    #[inline]
    fn new(image: &'a Image) -> Self {
        let dim = image.size();
        Self {
            data: &image.data,
            width: dim.x as u32,
            height: dim.y as u32,
        }
    }

    #[inline]
    const fn pixel_range(&self, x: u32, y: u32) -> Range<usize> {
        let pixel_size =
            (<Self as GenericImageView>::Pixel::CHANNEL_COUNT * u8::BITS as u8 / 8) as usize;
        let index = y as usize * self.width as usize * pixel_size + x as usize * pixel_size;
        Range {
            start: index,
            end: index + pixel_size,
        }
    }
}

impl<'a> GenericImageView for MutableImageView<'a> {
    type Pixel = Rgba<u8>;

    #[inline]
    fn dimensions(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    #[inline]
    fn bounds(&self) -> (u32, u32, u32, u32) {
        (0, 0, self.width, self.height)
    }

    #[inline]
    fn get_pixel(&self, x: u32, y: u32) -> Self::Pixel {
        let r = self.pixel_range(x, y);
        let pixel = &self.data[r];
        *Rgba::from_slice(pixel)
    }

    #[inline]
    fn width(&self) -> u32 {
        self.width
    }

    #[inline]
    fn height(&self) -> u32 {
        self.height
    }
}

pub struct MutableImage<'a> {
    data: &'a mut Vec<u8>,
    width: u32,
    height: u32,
}

impl<'a> MutableImage<'a> {
    #[inline]
    fn new(image: &'a mut Image) -> Self {
        let dim = image.size();
        Self {
            data: &mut image.data,
            width: dim.x as u32,
            height: dim.y as u32,
        }
    }

    #[inline]
    const fn pixel_range(&self, x: u32, y: u32) -> Range<usize> {
        let pixel_size =
            (<Self as GenericImageView>::Pixel::CHANNEL_COUNT * u8::BITS as u8 / 8) as usize;
        let index = y as usize * self.width as usize * pixel_size + x as usize * pixel_size;
        Range {
            start: index,
            end: index + pixel_size,
        }
    }
}

impl<'a> GenericImageView for MutableImage<'a> {
    type Pixel = Rgba<u8>;

    #[inline]
    fn dimensions(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    #[inline]
    fn bounds(&self) -> (u32, u32, u32, u32) {
        (0, 0, self.width, self.height)
    }

    #[inline]
    fn get_pixel(&self, x: u32, y: u32) -> Self::Pixel {
        let r = self.pixel_range(x, y);
        let pixel = &self.data[r];
        *Rgba::from_slice(pixel)
    }

    #[inline]
    fn width(&self) -> u32 {
        self.width
    }

    #[inline]
    fn height(&self) -> u32 {
        self.height
    }
}

impl<'a> GenericImage for MutableImage<'a> {
    #[inline]
    fn get_pixel_mut(&mut self, x: u32, y: u32) -> &mut Self::Pixel {
        let r = self.pixel_range(x, y);
        let pixel = &mut self.data[r];
        Rgba::from_slice_mut(pixel)
    }

    #[inline]
    #[allow(deprecated)]
    fn put_pixel(&mut self, x: u32, y: u32, pixel: Self::Pixel) {
        *self.get_pixel_mut(x, y) = pixel;
    }

    #[inline]
    #[allow(deprecated)]
    fn blend_pixel(&mut self, x: u32, y: u32, pixel: Self::Pixel) {
        let inner_pixel = self.get_pixel_mut(x, y);
        inner_pixel[0] = ((inner_pixel[0] as u16 * ((255 - pixel.0[3]) as u16)
            + pixel.0[0] as u16 * pixel.0[3] as u16)
            / 255u16) as u8;
    }
}
