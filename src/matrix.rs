use rand::{thread_rng, Rng};
use std::fmt::{Debug, Formatter, Result};

// 定义矩阵结构体
#[derive(Clone)]
pub struct Matrix {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<Vec<f64>>,
}

impl Matrix {
    /// 创建一个rows行cols列的全零矩阵
    pub fn zeros(rows: usize, cols: usize) -> Matrix {
        Matrix {
            rows,
            cols,
            data: vec![vec![0.0; cols]; rows],
        }
    }

    /// 创建一个指定大小的随机矩阵
    pub fn random(rows: usize, cols: usize) -> Matrix {
        // 获取当前线程的随机数生成器
        let mut rng = thread_rng();

        // 创建一个全零矩阵
        let mut res = Matrix::zeros(rows, cols);
        // 遍历矩阵的每个元素
        for i in 0..rows {
            for j in 0..cols {
                // 生成一个-1到1之间的随机数
                res.data[i][j] = rng.gen::<f64>() * 2.0 - 1.0;
            }
        }

        // 返回生成的随机矩阵
        res
    }

    // 此函数接受 f64 的向量向量并返回一个 Matrix
    pub fn from(data: Vec<Vec<f64>>) -> Matrix {
        // 使用输入数据的维度创建一个新矩阵
        Matrix {
            rows: data.len(),
            cols: data[0].len(),
            data,
        }
    }

    /// 实现矩阵乘法
    pub fn multiply(&self, other: &Matrix) -> Matrix {
        // 如果矩阵的列数不相等，则抛出异常
        if self.cols != other.rows {
            panic!("尝试乘以不正确维度的矩阵");
        }

        // 创建一个结果矩阵，并用0填充
        let mut res = Matrix::zeros(self.rows, other.cols);

        // 遍历self的行数和other的列数
        for i in 0..self.rows {
            for j in 0..other.cols {
                // 初始化一个变量，用于存储乘积
                let mut sum = 0.0;

                // 遍历self的列数和other的行数
                for k in 0..self.cols {
                    sum += self.data[i][k] * other.data[k][j];
                }

                // 将乘积赋值给结果矩阵
                res.data[i][j] = sum;
            }
        }

        // 返回结果矩阵
        res
    }

    /// 实现矩阵的加法
    pub fn add(&self, other: &Matrix) -> Matrix {
        // 如果矩阵的行列数不同，则抛出异常
        if self.rows != other.rows || self.cols != other.cols {
            panic!("Attempted to add matrix of incorrect dimensions");
        }

        // 创建一个新的矩阵，元素全为0
        let mut res = Matrix::zeros(self.rows, self.cols);

        // 遍历矩阵的每一个元素，将两个矩阵对应元素相加
        for i in 0..self.rows {
            for j in 0..self.cols {
                res.data[i][j] = self.data[i][j] + other.data[i][j];
            }
        }

        // 返回结果矩阵
        res
    }

    /// 矩阵点乘
    pub fn dot_multiply(&self, other: &Matrix) -> Matrix {
        // 检查两个矩阵的维度是否相同
        if self.rows != other.rows || self.cols != other.cols {
            panic!("Attempted to dot multiply by matrix of incorrect dimensions");
        }

        // 创建一个结果矩阵，初始化为0
        let mut res = Matrix::zeros(self.rows, self.cols);

        // 遍历矩阵的每个元素，进行点乘
        for i in 0..self.rows {
            for j in 0..self.cols {
                res.data[i][j] = self.data[i][j] * other.data[i][j];
            }
        }

        // 返回结果矩阵
        res
    }

    /// 按元素减去两个矩阵。
    ///
    /// # 恐慌
    ///
    /// 如果矩阵具有不同的维度，则会感到恐慌。
    pub fn subtract(&self, other: &Matrix) -> Matrix {
        if self.rows != other.rows || self.cols != other.cols {
            panic!("尝试减去不正确维度的矩阵");
        }

        let mut res = Matrix::zeros(self.rows, self.cols);

        for i in 0..self.rows {
            for j in 0..self.cols {
                res.data[i][j] = self.data[i][j] - other.data[i][j];
            }
        }

        res
    }

    /// 将函数应用于矩阵的每个元素，并返回一个包含结果的新矩阵。
    ///
    /// # Examples
    ///
    /// ```
    /// use matrix::Matrix;
    ///
    /// let m = Matrix::from([[1.0, 2.0], [3.0, 4.0]]);
    /// let doubled = m.map(|x| x * 2.0);
    ///
    /// assert_eq!(doubled, Matrix::from([[2.0, 4.0], [6.0, 8.0]]);
    /// ```
    pub fn map(&self, function: &dyn Fn(f64) -> f64) -> Matrix {
        // 从浮点数的二维向量创建新矩阵。
        Matrix::from(
            self.data
                .clone()
                .into_iter()
                //  将行的每个元素映射到应用函数的结果。
                .map(|row| row.into_iter().map(|value| function(value)).collect())
                //将行收集到二维向量中。
                .collect(),
        )
    }

    pub fn transpose(&self) -> Matrix {
        // 创建一个新的矩阵，行列互换
        let mut res = Matrix::zeros(self.cols, self.rows);

        // 遍历原矩阵
        for i in 0..self.rows {
            for j in 0..self.cols {
                // 将原矩阵的元素赋值给新矩阵
                res.data[j][i] = self.data[i][j];
            }
        }

        // 返回新矩阵
        res
    }
}

impl Debug for Matrix {
    // 实现Debug trait的fmt方法
    fn fmt(&self, f: &mut Formatter) -> Result {
        // 写入格式化字符串
        write!(
            f,
            "Matrix {{\n{}\n}}",
            (&self.data)
                // 遍历矩阵的每一行
                .into_iter()
                // 将每一行转换为字符串
                .map(|row| "  ".to_string()
                    + &row
                        // 遍历每一行的元素
                        .into_iter()
                        // 将元素转换为字符串
                        .map(|value| value.to_string())
                        // 将字符串元素连接成一个字符串
                        .collect::<Vec<String>>()
                        // 将字符串元素以空格分隔
                        .join(" "))
                // 将每一行的字符串连接成一个字符串
                .collect::<Vec<String>>()
                // 将每一行的字符串以换行符分隔
                .join("\n")
        )
    }
}
