use std::{
    fs::File,
    io::{Read, Write},
};

use serde::{Deserialize, Serialize};
use serde_json::{from_str, json};

use super::{activations::Activation, matrix::Matrix};

// 定义网络结构体
pub struct Network<'a> {
    layers: Vec<usize>,
    weights: Vec<Matrix>,
    biases: Vec<Matrix>,
    data: Vec<Matrix>,
    learning_rate: f64,
    activation: Activation<'a>,
}

// 定义一个结构体SaveData，用于存储权重和偏置
#[derive(Serialize, Deserialize)]
struct SaveData {
    // 存储权重
    weights: Vec<Vec<Vec<f64>>>,
    // 存储偏置
    biases: Vec<Vec<Vec<f64>>>,
}

impl Network<'_> {
    ///使用给定的层、学习率和激活函数创建一个新的神经网络。
    pub fn new(layers: Vec<usize>, learning_rate: f64, activation: Activation) -> Network {
        let mut weights = vec![];
        let mut biases = vec![];

        // Generate random weights and biases for each layer.
        for i in 0..layers.len() - 1 {
            weights.push(Matrix::random(layers[i + 1], layers[i]));
            biases.push(Matrix::random(layers[i + 1], 1));
        }

        Network {
            layers,
            weights,
            biases,
            data: vec![],
            learning_rate,
            activation,
        }
    }

    /// 根据输入，前向传播神经网络，并返回输出
    pub fn feed_forward(&mut self, inputs: Vec<f64>) -> Vec<f64> {
        // 检查输入长度是否正确
        if inputs.len() != self.layers[0] {
            panic!("Invalid inputs length");
        }

        // 将输入转换为矩阵，并将其转置
        let mut current = Matrix::from(vec![inputs]).transpose();
        // 将当前矩阵放入数据列表中
        self.data = vec![current.clone()];

        // 遍历网络的所有层，除了最后一层
        for i in 0..self.layers.len() - 1 {
            // 将当前矩阵乘以权重矩阵，并加上偏置矩阵，然后应用激活函数
            current = self.weights[i]
                .multiply(&current)
                .add(&self.biases[i])
                .map(self.activation.function);
            // 将当前矩阵放入数据列表中
            self.data.push(current.clone());
        }

        // 将输出矩阵转置，并返回输出向量
        current.transpose().data[0].to_owned()
    }

    /// 根据输出和目标，反向传播神经网络
    pub fn back_propagate(&mut self, outputs: Vec<f64>, targets: Vec<f64>) {
        // 检查目标长度是否正确
        if targets.len() != self.layers[self.layers.len() - 1] {
            panic!("Invalid targets length");
        }

        // 将输出转换为矩阵，并将其转置
        let parsed = Matrix::from(vec![outputs]).transpose();
        // 计算输出误差
        let mut errors = Matrix::from(vec![targets]).transpose().subtract(&parsed);
        // 计算输出梯度
        let mut gradients = parsed.map(self.activation.derivative);

        // 遍历网络的所有层，从最后一层开始
        for i in (0..self.layers.len() - 1).rev() {
            // 计算权重梯度和偏置梯度
            gradients = gradients
                .dot_multiply(&errors)
                .map(&|x| x * self.learning_rate);

            self.weights[i] = self.weights[i].add(&gradients.multiply(&self.data[i].transpose()));
            self.biases[i] = self.biases[i].add(&gradients);

            // 计算误差和梯度，用于下一层
            errors = self.weights[i].transpose().multiply(&errors);
            gradients = self.data[i].map(self.activation.derivative);
        }
    }

    /// 使用一组输入和目标训练神经网络。
    ///
    /// # Arguments
    /// * `inputs` - 输入向量的向量。
    /// * `targets` -目标向量的向量。
    /// * `epochs` - 运行训练数据的次数。
    pub fn train(&mut self, inputs: Vec<Vec<f64>>, targets: Vec<Vec<f64>>, epochs: u16) {
        // 循环遍历 epoch 数。
        for i in 1..=epochs {
            if epochs < 100 || i % (epochs / 100) == 0 {
                println!("Epoch {} of {}", i, epochs);
            }
            //遍历每个输入向量。
            for j in 0..inputs.len() {
                //  通过网络向前馈送输入向量并存储输出。
                let outputs = self.feed_forward(inputs[j].clone());
                //  通过网络将错误传播回去。
                self.back_propagate(outputs, targets[j].clone());
            }
        }
    }

    /// 保存神经网络到文件
    pub fn save(&self, file: String) {
        // 创建文件
        let mut file = File::create(file).expect("Unable to touch save file");

        // 将权重和偏置写入文件
        file.write_all(
            json!({
                "weights": self.weights.clone().into_iter().map(|matrix| matrix.data).collect::<Vec<Vec<Vec<f64>>>>(),
                "biases": self.biases.clone().into_iter().map(|matrix| matrix.data).collect::<Vec<Vec<Vec<f64>>>>()
            }).to_string().as_bytes(),
        ).expect("Unable to write to save file");
    }

    /// 从文件加载神经网络
    pub fn load(&mut self, file: String) {
        // 打开文件
        let mut file = File::open(file).expect("Unable to open save file");
        let mut buffer = String::new();

        // 读取文件内容
        file.read_to_string(&mut buffer)
            .expect("Unable to read save file");

        // 将文件内容反序列化为SaveData结构体
        let save_data: SaveData = from_str(&buffer).expect("Unable to serialize save data");

        // 创建权重和偏置向量
        let mut weights = vec![];
        let mut biases = vec![];

        // 从SaveData结构体中取出权重和偏置，并转换为矩阵
        for i in 0..self.layers.len() - 1 {
            weights.push(Matrix::from(save_data.weights[i].clone()));
            biases.push(Matrix::from(save_data.biases[i].clone()));
        }

        // 更新神经网络的权重和偏置
        self.weights = weights;
        self.biases = biases;
    }
}
