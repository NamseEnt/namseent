use nom::{bytes::complete::*, multi::*, number::complete::*, *};

struct PhotoshopFileFormat {
    file_header: FileHeaderSection,
    color_mode_data: ColorModeDataSection,
    image_resources: ImageResourcesSection,
    layer_and_mask_information: LayerAndMaskInformationSection,
    image_data: ImageDataSection,
}

struct FileHeaderSection {
    /// supports 1 ~ 56
    image_channel_count: u16,
    height: u32,
    width: u32,
    /// supports 1, 8, 16, 32
    depth_bits: u16,
    color_mode: ColorMode,
}

enum ColorMode {
    Bitmap,
    Grayscale,
    Indexed,
    Rgb,
    Cmyk,
    Multichannel,
    Duotone,
    Lab,
}

/// Not supported
struct ColorModeDataSection {}

struct ImageResourcesSection {}

struct ImageResourceBlock {}

struct LayerAndMaskInformationSection {}

struct ImageDataSection {}

fn photoshop_file_format(input: &[u8]) -> IResult<&[u8], PhotoshopFileFormat> {
    let (input, file_header) = file_header_section(input)?;
    let (input, color_mode_data) = color_mode_data_section(input)?;
    let (input, image_resources) = image_resources_section(input)?;
    let (input, layer_and_mask_information) = layer_and_mask_information_section(input)?;
    let (input, image_data) = image_data_section(input)?;

    Ok((
        input,
        PhotoshopFileFormat {
            file_header,
            color_mode_data,
            image_resources,
            layer_and_mask_information,
            image_data,
        },
    ))
}

fn file_header_section(input: &[u8]) -> IResult<&[u8], FileHeaderSection> {
    let (input, _) = tag("8BPS")(input)?;
    let (input, _) = tag([0, 1])(input)?;
    let (input, _) = tag([0, 0, 0, 0, 0, 0])(input)?;
    let (input, image_channel_count) = be_u16(input)?;

    let (input, height) = be_u32(input)?;
    let (input, width) = be_u32(input)?;
    let (input, depth_bits) = be_u16(input)?;
    let (input, color_mode) = be_u16(input)?;

    let color_mode = match color_mode {
        0 => ColorMode::Bitmap,
        1 => ColorMode::Grayscale,
        2 => ColorMode::Indexed,
        3 => ColorMode::Rgb,
        4 => ColorMode::Cmyk,
        7 => ColorMode::Multichannel,
        8 => ColorMode::Duotone,
        9 => ColorMode::Lab,
        _ => panic!("unsupported color mode"),
    };

    Ok((
        input,
        FileHeaderSection {
            image_channel_count,
            height,
            width,
            depth_bits,
            color_mode,
        },
    ))
}

fn color_mode_data_section(input: &[u8]) -> IResult<&[u8], ColorModeDataSection> {
    tag([0, 0, 0, 0])(input).map(|(input, _)| (input, ColorModeDataSection {}))
}

fn image_resources_section(input: &[u8]) -> IResult<&[u8], ImageResourcesSection> {
    // NOTE: section_byte_length seems not equal to the byte length of image_resources for clip-studio's psd exports.
    let (input, _section_byte_length) = be_u32(input)?;
    let (input, _image_resources) = many0(image_resource_block)(input)?;

    Ok((input, ImageResourcesSection {}))
}

fn image_resource_block(input: &[u8]) -> IResult<&[u8], ImageResourceBlock> {
    let (input, _signature) = tag("8BIM")(input)?;
    let (input, _resource_id) = be_u16(input)?;
    let (input, _name) = pascal_string(input, true)?;
    let (input, resource_byte_length) = be_u32(input)?;
    let (input, _resource_data) = take(resource_byte_length)(input)?;

    Ok((input, ImageResourceBlock {}))
}

fn pascal_string(input: &[u8], padding_even: bool) -> IResult<&[u8], ()> {
    let (input, length) = be_u8(input)?;
    let (input, string) = take(length)(input)?;
    let (input, _) = if padding_even && length % 2 == 1 {
        tag([0])(input)?
    } else {
        (input, &[] as &[u8])
    };

    Ok((input, ()))
}

fn layer_and_mask_information_section(
    input: &[u8],
) -> IResult<&[u8], LayerAndMaskInformationSection> {
    let (input, section_byte_length) = be_u32(input)?;
    if section_byte_length == 0 {
        return Ok((input, LayerAndMaskInformationSection {}));
    }

    let (input, _layer_info) = layer_info(input)?;
    let (input, _global_layer_mask_info) = global_layer_mask_info(input)?;
    let (input, _additional_layer_info) = additional_layer_info(input)?;

    Ok((input, LayerAndMaskInformationSection {}))
}

fn layer_info(input: &[u8]) -> IResult<&[u8], ()> {
    let (input, _layer_info_byte_length) = be_u32(input)?;
    let (input, layer_count) = be_i16(input)?;
    let first_alpha_channel_contains_transparency = layer_count < 0;
    let layer_count = layer_count.abs();
    let (input, _layer_info) = count(layer_record, layer_count as usize)(input)?;
    let (input, _channel_image_data) = many0(channel_image_data)(input)?;

    Ok((input, ()))
}

fn image_data_section(input: &[u8]) -> IResult<&[u8], ImageDataSection> {
    todo!()
}
