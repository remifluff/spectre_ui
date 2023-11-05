use std::fmt::format;

use slint::{slint, Model, SharedString};

slint::include_modules!();

fn main() {
    // println!("Hello, world!");
    let neo_matrix = MainWindow::new().unwrap();

    let rows = vec!["row_1", "row_2", "row_3", "row_4", "row_5"];
    let cols = vec!["col_1", "col_2", "col_3", "col_4", "col_5", "col_6", "col_7", "col_8", "col_9"];

    let mut nodes: Vec<NodeData> = vec![];

    for i in 0..rows.len() {
        for j in 0..cols.len() {
            nodes.push(NodeData { active: false, row_index: i as i32, col_index: j as i32 })
        }
    }

    nodes.len();

    neo_matrix.set_col_count(cols.len() as i32);
    neo_matrix.set_columns(slint_type_crap(cols).into());

    neo_matrix.set_row_count(rows.len() as i32);
    neo_matrix.set_rows(slint_type_crap(rows).into());

    // let mut nodes_model: Vec<NodeData> =
    // main_window.get_nodes().iter().collect();

    neo_matrix.set_nodes(std::rc::Rc::new(slint::VecModel::from(nodes)).into());

    let main_window_weak = neo_matrix.as_weak();

    // std::rc::Rc::new(slint::VecModel::from(tiles));

    println!("{}", "hi");

    neo_matrix.on_update(move |row, col| {
        let nodes_model: Vec<NodeData> = main_window_weak.unwrap().get_nodes().iter().collect();

        // println!("{}, {}", row, col);

        let binary_string = nodes_model
            .iter()
            .filter(|node| node.row_index == row)
            .map(|node| if node.active { "1" } else { "0" })
            .fold("".to_owned(), |acc, new_string| format!("{}{}", acc, new_string));

        let patch_integer = u64::from_str_radix(&binary_string, 2).unwrap();

        println!("D.M.{:02}.{:020}", row, patch_integer,);

        // let x = format!("{:0>64}", x);
        // println!("u64: {:020} - binary {:b}", x, x);
        // .for_each(|i| print!("{}", i));
    });

    neo_matrix.run().unwrap();
}

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
