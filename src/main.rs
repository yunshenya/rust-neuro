use crate::activations::SIGMOID;
use crate::network::Network;

pub mod activations;
pub mod matrix;
pub mod network;

fn main() {
    // 定义输入数据
    let inputs = vec![
        vec![0.0, 0.0],
        vec![0.0, 1.0],
        vec![1.0, 0.0],
        vec![1.0, 1.0],
    ];
    // 定义目标数据
    let targets = vec![vec![0.0], vec![1.0], vec![1.0], vec![0.0]];

    // 创建神经网络
    let mut network = Network::new(vec![2, 3, 1], 0.5, SIGMOID);

    // 训练神经网络
    network.train(inputs, targets, 3000);

    // 打印输出结果
    println!(
        "{:?}",
        network.feed_forward(vec![0.0, 0.0])
    );
    println!(
        "{:?}",
        network.feed_forward(vec![0.0, 1.0])
    );
    println!(
        "{:?}",
        network.feed_forward(vec![1.0, 0.0])
    );
    println!(
        "{:?}",
        network.feed_forward(vec![1.0, 1.0])
    );
}
