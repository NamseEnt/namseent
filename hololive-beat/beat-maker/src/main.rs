use aubio::{Notes, Smpl};

fn main() {
    let wave = get_wave();
    let sample_rate = 48000;

    let buf_size = 512;
    let hop_size = 4;

    let mut detector = microdsp::sfnov::SpectralFluxNoveltyDetector::new(buf_size);

    // let mut aubio = Notes::new(buf_size, hop_size, sample_rate).unwrap();
    let mut aubio =
        aubio::Onset::new(aubio::OnsetMode::Complex, buf_size, hop_size, sample_rate).unwrap();

    let period = 1.0 / sample_rate as Smpl;

    let mut time = 0.0;
    let mut offset = 0;

    wave.chunks(hop_size).for_each(|chunk| {
        // detector.process(chunk, |flux| {
        //     if flux.novelty() > 1.0 {
        //         println!("flux: {time}, novelty: {}", flux.novelty());
        //     }
        // });

        let note = aubio.do_result(chunk).unwrap();
        if note > 0.0 {
            println!("time: {time}, note: {}", note);
        }
        // for note in aubio.do_result(chunk).unwrap() {
        //     if note.velocity > 0.0 {
        //         println!(
        //             "time: {time}, pitch: {}, velocity: {}",
        //             note.pitch, note.velocity
        //         );
        //     }
        // }

        offset += hop_size;
        time = offset as Smpl * period;
    });

    println!("time: {time}");
}

fn get_wave() -> Vec<f32> {
    use symphonia::core::{
        codecs::CODEC_TYPE_NULL, formats::FormatOptions, io::MediaSourceStream,
        meta::MetadataOptions, probe::Hint,
    };

    let src = std::fs::File::open("../you-re-mine/kick.opus").expect("failed to read media");
    let mss = MediaSourceStream::new(Box::new(src), Default::default());

    let mut hint = Hint::new();
    hint.with_extension("ogg");
    let meta_opts: MetadataOptions = Default::default();
    let fmt_opts: FormatOptions = Default::default();

    let probed = symphonia::default::get_probe()
        .format(&hint, mss, &fmt_opts, &meta_opts)
        .expect("unsupported format");

    // Get the instantiated format reader.
    let mut format = probed.format;

    // Find the first audio track with a known (decodeable) codec.
    println!("track len: {}", format.tracks().len());

    let track = format
        .tracks()
        .iter()
        .find(|t| t.codec_params.codec != CODEC_TYPE_NULL)
        .expect("no supported audio tracks");

    println!("codec: {}", track.codec_params.codec);

    let mut decoder = opus::Decoder::new(48000, opus::Channels::Stereo).unwrap();
    let mut output = vec![];

    // Store the track identifier, it will be used to filter packets.
    let track_id = track.id;

    // The decode loop.
    while let Ok(packet) = format.next_packet() {
        // Consume any new metadata that has been read since the last packet.
        while !format.metadata().is_latest() {
            // Pop the old head of the metadata queue.
            format.metadata().pop();

            // Consume the new metadata at the head of the metadata queue.
        }

        // If the packet does not belong to the selected track, skip over it.
        if packet.track_id() != track_id {
            continue;
        }

        let mut chunk = [0.0f32; 1024 * 1024];
        let decoded = decoder
            .decode_float(&packet.data, &mut chunk, false)
            .unwrap();
        output.extend_from_slice(&chunk[..decoded]);
    }

    output
}
