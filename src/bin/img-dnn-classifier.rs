use anyhow::Result;
use opencv::{core, highgui, imgcodecs, prelude::*};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> Result<()> {
    let (w, h) = (640, 480);
    let window_name = "img-dnn-classifier";

    let mut net = opencv::dnn::read_net_from_caffe(
        "data/bvlc_googlenet.prototxt",
        "data/bvlc_googlenet.caffemodel",
    )?;

    // let img = opencv::imgcodecs::imread("data/small-cat.png", imgcodecs::IMREAD_COLOR)?;
    // let img = opencv::imgcodecs::imread("data/fish.png", imgcodecs::IMREAD_COLOR)?;
    let img = opencv::imgcodecs::imread("data/dog.png", imgcodecs::IMREAD_COLOR)?;

    let blob = opencv::dnn::blob_from_image(
        &img,
        1.,
        core::Size::new(224, 224),
        core::Scalar::new(104., 117., 123., 0.),
        false,
        false,
        core::CV_32F,
    )?;

    net.set_input_def(&blob)?;

    let mut output_blobs = core::Vector::<core::Mat>::new();
    let output_layer_names = net.get_unconnected_out_layers_names()?;
    net.forward(&mut output_blobs, &output_layer_names)?;

    let synset_words = get_synset_words("data/synset_words.txt")?;

    // 获取匹配最高的3项及其索引
    let output_probabilities = output_blobs.get(0).unwrap().to_vec_2d::<f32>()?;
    let probabilities = output_probabilities.get(0).unwrap();

    // 创建一个包含(概率, 索引)的向量
    let mut prob_index: Vec<(f32, usize)> = probabilities
        .iter()
        .enumerate()
        .map(|(i, &prob)| (prob, i))
        .collect();

    // 按概率降序排序
    prob_index.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());

    // 获取前3个最高概率的索引
    let top3_indices: Vec<usize> = prob_index.iter().take(3).map(|&(_, idx)| idx).collect();

    // 从synset_words中获取对应的类别描述
    println!("Top 3 predictions:");
    for (i, &idx) in top3_indices.iter().enumerate() {
        if let Some((_, class_desc)) = synset_words.get(idx) {
            let prob = prob_index[i].0;
            println!("{:.2}%: {}", prob * 100.0, class_desc);
        }
    }

    highgui::named_window(window_name, highgui::WINDOW_NORMAL)?;
    highgui::resize_window(window_name, w, h)?;
    highgui::imshow(window_name, &img)?;

    loop {
        let key = highgui::wait_key(0)?;
        if key & 0xFF == 'q' as i32 {
            break;
        }
    }

    highgui::destroy_all_windows()?;
    Ok(())
}

fn get_synset_words<P>(filename: P) -> io::Result<Vec<(String, String)>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);

    let mut result = Vec::new();

    for line in reader.lines() {
        let line = line?;
        if let Some(split_index) = line.find(' ') {
            let (first_part, second_part) = line.split_at(split_index);
            let second_part = second_part.trim_start();
            result.push((first_part.to_string(), second_part.to_string()));
        }
    }

    Ok(result)
}
