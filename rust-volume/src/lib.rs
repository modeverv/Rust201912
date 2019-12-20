#[macro_use]
extern crate vst;

use vst::buffer::AudioBuffer;
use vst::plugin::{Info, Plugin,PluginParameters};
use std::sync::Arc;

#[derive(Default)]
struct Whisper {
    // パラメーターをここに保存する
    // 後でArcで渡すのでArcで保存する
    params: Arc<WhisperParameters>,
    // Added a counter in our plugin struct. 
    notes: u8
}

// パラメーターの中身。ボリュームを制御するf32だけ
struct WhisperParameters {
    // アトミックなf32、内部ではstd::sync::atomic::AtomicU32を使っている。
    // VSTの仕様のためパラメーターはすべてf32で値の範囲は0~1でなければならない
    volume: vst::util::AtomicFloat,
}

impl Default for WhisperParameters {
    fn default() -> Self {
        Self {
            volume: vst::util::AtomicFloat::new(1.0),
        }
    }
}

impl PluginParameters for WhisperParameters {
    fn get_parameter_label(&self, index: i32) -> String {
        match index {
            // 適当にラベルを返す
            0 => "x".to_string(),
            _ => "".to_string(),
        }
    }
    // This is what will display underneath our control.  We can
    // format it into a string that makes the most sense.
    fn get_parameter_text(&self, index: i32) -> String {
        match index {
            0 => format!("{:.3}", self.volume.get()),
            _ => format!(""),
        }
    }

    fn get_parameter_name(&self, index: i32) -> String {
        match index {
            0 => "volume".to_string(),
            _ => "".to_string(),
        }
    }
    // get_parameter has to return the value used in set_parameter
    fn get_parameter(&self, index: i32) -> f32 {
        match index {
            0 => self.volume.get(),
            _ => 0.0,
        }
    }
    fn set_parameter(&self, index: i32, value: f32) {
        match index {
            0 => self.volume.set(value),
            _ => (),
        }
    }
}

impl Plugin for Whisper {
    fn get_info(&self) -> Info {
        Info {
                name: "Rust Volume Plugin".to_string(),
                unique_id: 8050, // Used by hosts to differentiate between plugins.

            // このプラグインで使うパラメータ数を設定する
            parameters: 1,
            ..Default::default()
        }
    }

    fn process(&mut self, buffer: &mut AudioBuffer<f32>) {

        // `buffer.split()` gives us a tuple containing the 
        // input and output buffers.  We only care about the
        // output, so we can ignore the input by using `_`.
        let (_, mut output_buffer) = buffer.split();

        // We only want to process *anything* if a note is being held.
        // Else, we can fill the output buffer with silence.
        /*
        if self.notes == 0 {
            for output_channel in output_buffer.into_iter() {
                // Let's iterate over every sample in our channel.
                for output_sample in output_channel {
                    *output_sample = 0.0;
                }
            }
            return;
        }
        */

        let volume = self.params.volume.get();

        // Now, we want to loop over our output channels.  This
        // includes our left and right channels (or more, if you
        // are working with surround sound).
        for output_channel in output_buffer.into_iter() {
            // Let's iterate over every sample in our channel.
            for output_sample in output_channel {
                // For every sample, we want to generate a random value
                // from -1.0 to 1.0.
                // ここでボリュームを掛けて音の大きさを調整する
                *output_sample = volume;
            }
        }
    }

    fn get_parameter_object(&mut self) -> Arc<dyn PluginParameters> {
        Arc::clone(&self.params) as Arc<dyn PluginParameters>
    }
}

plugin_main!(Whisper);