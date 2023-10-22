use slint::{slint, SharedString};

fn main() {
    // println!("Hello, world!");
    let main_window = MainWindow::new().unwrap();

    let rows = vec!["row_1", "row_2", "row_3", "row_4", "row_5"];
    let cols = vec!["col_1", "col_2", "yo"];

    let mut nodes: Vec<NodeData> = vec![];

    for i in 0..rows.len() {
        for j in 0..cols.len() {
            nodes.push(NodeData { active: false, row_index: i as i32, col_index: j as i32 })
        }
    }

    nodes.len();
    // let col_names = std::rc::Rc::new
    main_window.set_col_count(cols.len() as i32);
    main_window.set_columns(slint_type_crap(cols).into());

    main_window.set_row_count(rows.len() as i32);
    main_window.set_rows(slint_type_crap(rows).into());

    main_window.set_nodes(std::rc::Rc::new(slint::VecModel::from(nodes)).into());

    let main_window_weak = main_window.as_weak();

    main_window.on_update(move || {});

    main_window.run().unwrap();
}
slint!( import {MainWindow} from "ui/ui.slint";);

fn slint_type_crap(names: Vec<&str>) -> std::rc::Rc<slint::VecModel<SharedString>> {
    std::rc::Rc::new(slint::VecModel::from(
        names.iter().map(|name| Into::<SharedString>::into(*name)).collect::<Vec<SharedString>>(),
    ))
}
// let cols = vec![
//         "invert_x_0",
//         "invert_x_1",
//         "invert_x_2",
//         "invert_x_3",
//         "invert_x_4",
//         "invert_x_5",
//         "invert_x_6",
//         "invert_x_7",
//         "invert_x_8",
//         "invert_y_0",
//         "invert_y_1",
//         "invert_y_2",
//         "invert_y_3",
//         "invert_y_4",
//         "invert_y_5",
//         "invert_y_6",
//         "invert_y_7",
//         "invert_y_8",
//         "overlay_gate_1_dis",
//         "overlay_gate_1_sig",
//         "overlay_gate_2_dis",
//         "overlay_gate_2_sig",
//         "overlay_gate_3_dis",
//         "overlay_gate_3_sig",
//         "overlay_gate_4_dis",
//         "overlay_gate_4_sig",
//         "invert_a",
//         "invert_b",
//         "invert_c",
//         "invert_d",
//         "edge",
//         "delay",
//         "flip_flop_+",
//         "flip_flop_-",
//         "to_acm_fast",
//         "to_acm_slow",
//         "out_a_luma_0",
//         "out_a_luma_1",
//         "out_a_luma_2",
//         "out_a_luma_3",
//         "out_a_col1_0",
//         "out_a_col1_1",
//         "out_a_col1_2",
//         "out_a_col2_0",
//         "out_a_col2_1",
//         "out_a_col2_2",
//         "out_b_luma_0",
//         "out_b_luma_1",
//         "out_b_luma_2",
//         "out_b_luma_3",
//         "out_b_col1_0",
//         "out_b_col1_1",
//         "out_b_col1_2",
//         "out_b_col2_0",
//         "out_b_col2_1",
//         "out_b_col2_2",
//         "col_swap",
//     ];
//     let rows = vec![
//         "counter_x_0",
//         "counter_x_1",
//         "counter_x_2",
//         "counter_x_3",
//         "counter_x_4",
//         "counter_x_5",
//         "counter_x_6",
//         "counter_x_7",
//         "counter_x_8",
//         "counter_y_0",
//         "counter_y_1",
//         "counter_y_2",
//         "counter_y_3",
//         "counter_y_4",
//         "counter_y_5",
//         "counter_y_6",
//         "counter_y_7",
//         "counter_y_8",
//         "slow_count_6",
//         "slow_count_3",
//         "slow_count_1.5",
//         "slow_count_.8",
//         "slow_count_.4",
//         "slow_count_.2",
//         "overlay_1",
//         "overlay_2",
//         "overlay_3",
//         "overlay_4",
//         "invert_a",
//         "invert_b",
//         "invert_c",
//         "invert_d",
//         "edge_thin_+",
//         "edge_thin_-",
//         "edge_wide_+",
//         "edge_wide_-",
//         "delay",
//         "flip_flop_+",
//         "flip_flop_-",
//         "n/a",
//         "n/a",
//         "n/a",
//         "n/a",
//         "comp_0",
//         "comp_1",
//         "comp_2",
//         "comp_3",
//         "comp_4",
//         "comp_5",
//         "comp_6",
//     ];
