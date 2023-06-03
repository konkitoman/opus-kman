use bytes_kman::TBytes;
use opus_kman::packet::Packet;

fn ilog(a: f64) -> f64 {
    a.log2().floor() + 1.0
}

fn main() {
    let host = cpal::default_host();

    unsafe {
        let mut error = 0;
        // 8, 12, 16, 24, 48
        const freq: i32 = 8;
        // 1, 2
        const channels: i32 = 1;
        let encoder = audiopus_sys::opus_encoder_create(
            freq * 1000,
            channels,
            audiopus_sys::OPUS_APPLICATION_AUDIO,
            &mut error,
        );
        let input = [0; freq as usize * 20 * channels as usize];
        let mut output = vec![0; 4096];
        let res = audiopus_sys::opus_encode(
            encoder,
            input.as_ptr(),
            input.len() as i32 / channels,
            output.as_mut_ptr(),
            output.len() as i32,
        );
        let mut out = output[..res as usize].to_vec();
        let pak = Packet::from_bytes(&mut out).unwrap();
        println!("Packet: {pak:?}");

        let output: String = out
            .iter()
            .map(|b| format!("{:08b}", b))
            .collect::<Vec<String>>()
            .join(", ");
        println!("Out: {output}");
    }
}
