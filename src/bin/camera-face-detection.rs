use anyhow::Result;
use opencv::{
    core::{self, Mat, Rect, Vector},
    highgui, imgproc, objdetect,
    prelude::*,
    videoio::{VideoCapture, CAP_ANY},
};

fn main() -> Result<()> {
    // 初始化摄像头
    let mut ready_index = Vector::new();
    let mut caps = Vector::new();
    caps.push(VideoCapture::new(0, CAP_ANY)?);

    // 检测可用摄像头
    if !VideoCapture::wait_any(&caps, &mut ready_index, 0)? {
        println!("无法找到摄像头");
        return Ok(());
    }

    println!("可用的摄像头索引: {ready_index:?}");

    // 为每个摄像头创建配置
    let mut entries = vec![];
    for (index, cap) in caps.iter().enumerate() {
        let width = cap.get(opencv::videoio::CAP_PROP_FRAME_WIDTH)? as i32;
        let height = cap.get(opencv::videoio::CAP_PROP_FRAME_HEIGHT)? as i32;
        let fps = cap.get(opencv::videoio::CAP_PROP_FPS)?;
        let fps = if fps <= 0.0 { 30.0 } else { fps };
        let delay = (1000.0 / fps) as i32;

        println!(
            "摄像头 {}: 分辨率({}x{}), FPS: {}",
            index, width, height, fps
        );
        let window_name = format!("摄像头-{}", index);
        highgui::named_window(&window_name, highgui::WINDOW_NORMAL)?;
        highgui::resize_window(&window_name, width, height)?;
        entries.push((window_name, delay));
    }

    // 加载人脸检测模型 (使用LBP模型，更快且对光照变化更鲁棒)
    let mut face_classifier =
        opencv::objdetect::CascadeClassifier::new("data/haarcascade_frontalface_default.xml")?;
    // objdetect::CascadeClassifier::new("data/lbpcascade_frontalface_improved.xml")?;

    let mut face_history = std::collections::HashMap::new();

    // 主循环
    'out: loop {
        for index in ready_index.iter() {
            let start = std::time::Instant::now();
            let index = index as usize;
            let mut img = Mat::default();

            if let Ok(true) = caps.get(index).unwrap().read(&mut img) {
                // 水平翻转使画面更自然
                core::flip(&img.clone(), &mut img, 1)?;

                // 调整亮度和对比度
                let mut adjusted = Mat::default();
                img.convert_to(&mut adjusted, -1, 1.2, 30.0)?;

                // 转换为灰度图
                let mut gray = Mat::default();
                imgproc::cvt_color(&adjusted, &mut gray, imgproc::COLOR_BGR2GRAY, 0)?;

                // 直方图均衡化增强对比度
                imgproc::equalize_hist(&gray.clone(), &mut gray)?;

                // 轻微高斯模糊减少噪声
                let mut blurred = Mat::default();
                imgproc::gaussian_blur(
                    &gray,
                    &mut blurred,
                    core::Size::new(3, 3),
                    0.0,
                    0.0,
                    core::BORDER_DEFAULT,
                )?;

                // highgui::imshow(&entries[index].0, &blurred)?;

                // 检测人脸
                let mut objs = Vector::<Rect>::new();
                face_classifier.detect_multi_scale(
                    &blurred,
                    &mut objs,
                    1.05,                      // 缩小比例因子
                    3,                         // 最小邻居数
                    0,                         // 标志
                    core::Size::new(60, 60),   // 最小人脸尺寸
                    core::Size::new(300, 300), // 最大人脸尺寸
                )?;

                // 非极大值抑制减少重叠框
                objdetect::group_rectangles(&mut objs, 0, 0.2)?;

                println!("{}", objs.len());

                // 多帧验证 - 更新人脸历史记录
                let mut current_faces = std::collections::HashSet::new();
                for rect in objs.iter() {
                    let center = (rect.x + rect.width / 2, rect.y + rect.height / 2);
                    current_faces.insert(center);
                }

                // 更新历史记录, 记录在多帧中基本不移动的人脸
                face_history.retain(|center: &(i32, i32), count: &mut i32| {
                    let mut found = false;
                    for &current_center in &current_faces {
                        let dist = ((center.0 - current_center.0).pow(2)
                            + (center.1 - current_center.1).pow(2))
                            as f32;
                        if dist < 100.0 {
                            *count += 1; // 出现的帧数
                            found = true;
                            break;
                        }
                    }
                    found
                });

                // 添加新检测到的人脸
                for &center in &current_faces {
                    if !face_history.contains_key(&center) {
                        face_history.insert(center, 1);
                    }
                }

                // 只绘制在多帧中持续出现的人脸
                for rect in objs.into_iter() {
                    let center = (rect.x + rect.width / 2, rect.y + rect.height / 2);
                    if let Some(count) = face_history.get(&center) {
                        // 至少连续3帧检测到
                        let color = if *count > 10 {
                            // 持续检测到的人脸用绿色
                            core::Scalar::new(0., 255., 0., 0.)
                        } else if *count > 2 {
                            // 新检测到的人脸用蓝色
                            core::Scalar::new(255., 0., 0., 0.)
                        } else {
                            // 新检测到的人脸用红色
                            core::Scalar::new(0., 0., 255., 0.)
                        };

                        imgproc::rectangle(&mut img, rect, color, 2, imgproc::LINE_8, 0)?;

                        // 显示置信度(基于持续帧数)
                        let label =
                            format!("Conf: {:.1}%", (*count as f32 / 10.0).min(1.0) * 100.0);

                        imgproc::put_text(
                            &mut img,
                            &label,
                            core::Point::new(rect.x, rect.y - 5),
                            imgproc::FONT_HERSHEY_SIMPLEX,
                            0.5,
                            color,
                            1,
                            imgproc::LINE_AA,
                            false,
                        )?;
                    }
                }

                // 显示处理后的图像
                highgui::imshow(&entries[index].0, &img)?;
            }

            // 计算处理时间并调整等待时间
            let elapsed = start.elapsed().as_millis() as i32;
            let wait_time = std::cmp::max(1, entries[index].1 - elapsed);

            // 检查退出键
            let key = highgui::wait_key(wait_time)?;
            if key & 0xFF == 'q' as i32 {
                break 'out;
            }
        }
    }

    // 清理资源
    for mut cap in caps.into_iter() {
        cap.release()?;
    }
    highgui::destroy_all_windows()?;

    Ok(())
}
