use image::{ImageBuffer, Rgba};
use windows::{
    Storage::Streams::{
        DataReader, 
        IRandomAccessStream, 
        InputStreamOptions
    }, 
    Win32::{
        Graphics::Gdi::{
            BI_RGB, BITMAP, BITMAPINFO, BITMAPINFOHEADER, DIB_RGB_COLORS, DeleteObject, GetDC,
            GetDIBits, GetObjectW, HGDIOBJ, RGBQUAD, ReleaseDC,
        },
        UI::WindowsAndMessaging::{DestroyIcon, GetIconInfo, HICON, ICONINFO},
    }
};

/// Convert an HICON to PNG bytes using the same bitmap extraction pipeline.
#[cfg_attr(feature = "profiling", tracing::instrument)]
pub fn hicon_to_png_bytes(hicon: HICON) -> Result<Vec<u8>, String> {
    let mut icon_info = ICONINFO::default();
    unsafe { GetIconInfo(hicon, &mut icon_info) }.map_err(|_| "GetIconInfo failed".to_string())?;
    let mut bmp = BITMAP::default();

    if unsafe {
        GetObjectW(
            HGDIOBJ(icon_info.hbmColor.0),
            std::mem::size_of::<BITMAP>() as i32,
            Some(&mut bmp as *mut _ as *mut _),
        )
    } == 0
    {
        return Err("GetObjectW failed".into());
    }

    let (width, height) = (bmp.bmWidth as u32, bmp.bmHeight as u32);
    let mut buffer = vec![0u8; (width * height * 4) as usize];

    let mut bmi = BITMAPINFO {
        bmiHeader: BITMAPINFOHEADER {
            biSize: std::mem::size_of::<BITMAPINFOHEADER>() as u32,
            biWidth: width as i32,
            biHeight: -(height as i32),
            biPlanes: 1,
            biBitCount: 32,
            biCompression: BI_RGB.0 as u32,
            biSizeImage: 0,
            biXPelsPerMeter: 0,
            biYPelsPerMeter: 0,
            biClrUsed: 0,
            biClrImportant: 0,
        },
        bmiColors: [RGBQUAD::default(); 1],
    };

    let hdc = unsafe { GetDC(None) };
    let res = unsafe {
        GetDIBits(
            hdc,
            icon_info.hbmColor,
            0,
            height,
            Some(buffer.as_mut_ptr() as _),
            &mut bmi,
            DIB_RGB_COLORS,
        )
    };
    unsafe { ReleaseDC(None, hdc) };
    if res == 0 {
        return Err("GetDIBits failed".into());
    }

    for px in buffer.chunks_exact_mut(4) {
        px.swap(0, 2);
    }

    let img_buf: ImageBuffer<Rgba<u8>, Vec<u8>> =
        ImageBuffer::from_raw(width, height, buffer).ok_or("Image buffer creation failed")?;
    let mut png_bytes = Vec::new();
    image::DynamicImage::ImageRgba8(img_buf)
        .write_to(
            &mut std::io::Cursor::new(&mut png_bytes),
            image::ImageFormat::Png,
        )
        .map_err(|e| format!("Error writing PNG: {e}"))?;

    unsafe {
        DeleteObject(icon_info.hbmColor.into())
            .ok()
            .map_err(|e| format!("{e}"))?;
        DeleteObject(icon_info.hbmMask.into())
            .ok()
            .map_err(|e| format!("{e}"))?;
        DestroyIcon(hicon).map_err(|e| format!("{e}"))?;
    }

    Ok(png_bytes)
}

pub fn stream_to_png_bytes(stream: &IRandomAccessStream) -> Result<Vec<u8>, String> {
    let size = stream
        .Size()
        .map_err(|e| format!("Error obtaining stream siz: {e}"))?
        .try_into()
        .map_err(|e| format!("Error converting u64 to u32: {e}"))?;

    let input_stream = stream
        .GetInputStreamAt(0)
        .map_err(|e| format!("Error obtaining input stream: {e}"))?;

    let reader = DataReader::CreateDataReader(&input_stream)
        .map_err(|e| format!("Error creating data reader: {e}"))?;

    reader.LoadAsync(size).map_err(|e| format!("Error loading bytes: {e}"))?;
    let mut bytes = vec![0u8; size as usize];

    reader.ReadBytes(&mut bytes).map_err(|e| format!("Error reading bytes: {e}"))?;

    Ok(bytes)
}