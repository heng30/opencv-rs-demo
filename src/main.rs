use anyhow::Result;
use opencv::{core, highgui, imgcodecs, imgproc, prelude::*};

fn main() -> Result<()> {
    let (w, h) = (640, 480);
    let window_name = "img-card-number";

    /******************** 对模板数字进行处理 *******************/
    let mut img_template_orig =
        imgcodecs::imread("data/credit-number.png", imgcodecs::IMREAD_COLOR)?;
    let mut img_template = Mat::default();

    opencv::imgproc::cvt_color(
        &img_template_orig,
        &mut img_template,
        opencv::imgproc::ColorConversionCodes::COLOR_BGR2GRAY.into(),
        0,
        opencv::core::AlgorithmHint::ALGO_HINT_DEFAULT.into(),
    )?;

    // 二值化
    opencv::imgproc::threshold(
        &img_template.clone(),
        &mut img_template,
        10.,
        255.,
        opencv::imgproc::THRESH_BINARY_INV,
    )?;

    // 查找轮廓
    let mut contours = core::Vector::<core::Mat>::new();
    imgproc::find_contours(
        &img_template,
        &mut contours,
        imgproc::RETR_EXTERNAL,
        imgproc::CHAIN_APPROX_SIMPLE,
        core::Point::default(),
    )?;

    // 获取每个数字的包围矩形
    let mut bounding_rects = vec![];
    for contour in contours.into_iter() {
        let bounding_rect = opencv::imgproc::bounding_rect(&contour)?;
        bounding_rects.push(bounding_rect.clone());

        // 绘制包围进行
        opencv::imgproc::rectangle(
            &mut img_template_orig,
            bounding_rect,
            opencv::core::Scalar::new(0., 0., 255., 0.),
            2,
            opencv::imgproc::LINE_4,
            0,
        )?;
    }
    assert_eq!(10, bounding_rects.len());

    // 从小到大遍历(0-9)，获取每个数字的截图
    let mut numbers_roi = vec![];
    for (_index, rect) in bounding_rects.into_iter().rev().enumerate() {
        numbers_roi.push(Mat::roi(&img_template, rect)?);
    }

    /******************** 对银行卡进行处理 *******************/
    let img_orig = imgcodecs::imread("data/credit-card.png", imgcodecs::IMREAD_COLOR)?;
    let mut img = Mat::default();

    opencv::imgproc::cvt_color(
        &img_orig,
        &mut img,
        opencv::imgproc::ColorConversionCodes::COLOR_BGR2GRAY.into(),
        0,
        opencv::core::AlgorithmHint::ALGO_HINT_DEFAULT.into(),
    )?;

    let kernel = opencv::imgproc::get_structuring_element(
        opencv::imgproc::MorphShapes::MORPH_RECT.into(),
        core::Size::new(9, 3),
        core::Point::new(-1, -1),
    )?;

    // 原图 - 开运算 -> 获取主体外的噪声
    opencv::imgproc::morphology_ex(
        &img.clone(),
        &mut img,
        opencv::imgproc::MORPH_TOPHAT,
        &kernel,
        core::Point::new(-1, -1),
        5,
        core::BORDER_CONSTANT,
        opencv::imgproc::morphology_default_border_value()?,
    )?;

    // 查找轮廓
    let mut gradient_x = Mat::default();
    let mut gradient_y = Mat::default();
    opencv::imgproc::sobel_def(&img, &mut gradient_x, -1, 1, 0)?;
    opencv::imgproc::sobel_def(&img, &mut gradient_y, -1, 0, 1)?;
    let mut img_gradient = (gradient_x + gradient_y).into_result()?.to_mat()?;

    // 二值化
    opencv::imgproc::threshold(
        &img.clone(),
        &mut img,
        100.,
        255.,
        opencv::imgproc::THRESH_BINARY,
    )?;

    // 膨胀,将相邻的数字连成一个整体
    let dilate_kernel = opencv::imgproc::get_structuring_element(
        opencv::imgproc::MorphShapes::MORPH_CROSS.into(),
        core::Size::new(5, 5),
        core::Point::new(-1, -1),
    )?;

    opencv::imgproc::dilate(
        &img_gradient.clone(),
        &mut img_gradient,
        &dilate_kernel,
        core::Point::new(-1, -1),
        7,
        core::BORDER_CONSTANT,
        opencv::imgproc::morphology_default_border_value()?,
    )?;

    // 二值化
    opencv::imgproc::threshold(
        &img_gradient.clone(),
        &mut img_gradient,
        100.,
        255.,
        opencv::imgproc::THRESH_BINARY,
    )?;

    let mut contours = core::Vector::<core::Mat>::new();
    imgproc::find_contours(
        &img_gradient,
        &mut contours,
        imgproc::RETR_EXTERNAL,
        imgproc::CHAIN_APPROX_SIMPLE,
        core::Point::default(),
    )?;

    // 获取每个数字的包围矩形
    let mut bounding_rects = vec![];
    for contour in contours.into_iter() {
        let bounding_rect = opencv::imgproc::bounding_rect(&contour)?;
        bounding_rects.push(bounding_rect.clone());
    }

    // 查找可能的数字块开始下标，一共有4个数字块
    let size_delta = 10;
    let mut tmp_index = 0;
    let mut block_count = 0;
    for index in 1..bounding_rects.len() {
        let min_w = bounding_rects[tmp_index]
            .width
            .min(bounding_rects[index].width);

        let min_h = bounding_rects[tmp_index]
            .height
            .min(bounding_rects[index].height);

        let max_w = bounding_rects[tmp_index]
            .width
            .max(bounding_rects[index].width);

        let max_h = bounding_rects[tmp_index]
            .height
            .max(bounding_rects[index].height);

        // 目标4个数字块的高度和宽度基本相同
        if max_w - min_w < size_delta && max_h - min_h < size_delta {
            block_count += 1;

            if block_count == 3 {
                tmp_index = index - 3;
                break;
            }
        } else {
            block_count = 0;
        }

        tmp_index = index;
    }

    // 找到四个大小相近的包围矩形
    assert_ne!(tmp_index, bounding_rects.len() - 1);

    let bounding_rects = bounding_rects[tmp_index..tmp_index + 4]
        .into_iter()
        .cloned()
        .collect::<Vec<_>>();

    // 获取信用卡数字截图
    let mut card_numbers_roi = vec![];
    for (_index, rect) in bounding_rects.into_iter().rev().enumerate() {
        card_numbers_roi.push(Mat::roi(&img, rect)?);
        // highgui::imshow(
        //     &format!("{window_name}-{_index}"),
        //     &card_numbers_roi.last().unwrap(),
        // )?;
    }

    // 将4组信用卡块切分为单数字截图
    let mut card_all_numbers_roi = vec![];
    for roi in card_numbers_roi.iter() {
        let mut contours = core::Vector::<core::Mat>::new();
        imgproc::find_contours(
            &roi,
            &mut contours,
            imgproc::RETR_EXTERNAL,
            imgproc::CHAIN_APPROX_SIMPLE,
            core::Point::default(),
        )?;

        // 获取每个数字的包围矩形
        let mut bounding_rects = vec![];
        for contour in contours.into_iter() {
            let bounding_rect = opencv::imgproc::bounding_rect(&contour)?;
            bounding_rects.push(bounding_rect.clone());
        }

        assert_eq!(4, bounding_rects.len());

        // 从左到右遍历，获取每个数字的截图
        for (_index, rect) in bounding_rects.into_iter().rev().enumerate() {
            card_all_numbers_roi.push(Mat::roi(roi, rect)?);
        }
    }

    // 显示信用卡所有数字
    // for (index, roi) in card_all_numbers_roi.iter().enumerate() {
    //     highgui::imshow(&format!("card-number-{index}"), roi)?;
    // }

    // 显示所有模板数字
    // for (index, roi) in numbers_roi.iter().enumerate() {
    //     highgui::imshow(&format!("number-{index}"), roi)?;
    // }

    // 模板数字和信用卡数字进行匹配
    for card_number in card_all_numbers_roi.iter() {
        let size = card_number.size()?;
        for (index, number) in numbers_roi.iter().enumerate() {
            let mut number_copy = Mat::default();

            opencv::imgproc::resize(
                &number,
                &mut number_copy,
                size,
                0.,
                0.,
                opencv::imgproc::INTER_AREA,
            )?;

            let mut result = Mat::default();
            opencv::imgproc::match_template_def(
                &card_number,
                &number_copy,
                &mut result,
                opencv::imgproc::TM_CCOEFF_NORMED,
            )?;

            let mut max_val = 0.0_f64;
            core::min_max_loc(
                &result,
                None,
                Some(&mut max_val),
                None,
                None,
                &core::no_array(),
            )?;

            // 概率大于0.8的，认为匹配成功
            if max_val > 0.8 {
                println!("Card number: {index}, max probability: {max_val:.2}, ");
            }
        }
    }

    highgui::named_window(window_name, highgui::WINDOW_NORMAL)?;
    highgui::resize_window(window_name, w, h)?;
    highgui::imshow(window_name, &img_orig)?;

    loop {
        let key = highgui::wait_key(0)?;
        if key & 0xFF == 'q' as i32 {
            break;
        }
    }

    highgui::destroy_all_windows()?;
    Ok(())
}
