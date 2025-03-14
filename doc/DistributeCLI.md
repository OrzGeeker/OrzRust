要将使用Rust编写的CLI应用发布到Homebrew并支持多平台用户使用，需结合交叉编译、GitHub Releases分发和Homebrew Formula配置。以下是具体步骤：

---

### **1. 交叉编译生成多平台可执行文件**
- **安装目标平台工具链**  
  使用`rustup`添加目标平台的编译支持，例如：  
  ```bash
  # macOS (Intel/Apple Silicon)
  rustup target add x86_64-apple-darwin aarch64-apple-darwin
  # Linux
  rustup target add x86_64-unknown-linux-gnu
  # Windows (可选，若需支持)
  rustup target add x86_64-pc-windows-gnu
  ```

- **安装交叉编译工具**  
  - 对Linux目标，通过Homebrew安装`musl-cross`：  
    ```bash
    brew install FiloSottile/musl-cross/musl-cross
    ```
  - 对Windows目标，安装`mingw-w64`：  
    ```bash
    brew install mingw-w64
    ```

- **编译各平台二进制文件**  
  通过`cargo build --target`指定目标平台，例如：  
  ```bash
  cargo build --release --target x86_64-apple-darwin    # macOS Intel
  cargo build --release --target aarch64-apple-darwin   # macOS Apple Silicon
  cargo build --release --target x86_64-unknown-linux-gnu # Linux
  ```

---

### **2. 打包并上传至GitHub Releases**
- **为每个平台生成压缩包**  
  将编译后的可执行文件（如`my_cli`）按平台打包：  
  ```bash
  # macOS Intel
  tar -czf my_cli-x86_64-apple-darwin.tar.gz target/x86_64-apple-darwin/release/my_cli
  # macOS Apple Silicon
  tar -czf my_cli-aarch64-apple-darwin.tar.gz target/aarch64-apple-darwin/release/my_cli
  # Linux
  tar -czf my_cli-x86_64-unknown-linux-gnu.tar.gz target/x86_64-unknown-linux-gnu/release/my_cli
  ```

- **计算哈希值**  
  使用`shasum`生成SHA256校验和：  
  ```bash
  shasum -a 256 my_cli-*.tar.gz
  ```

- **上传至GitHub Releases**  
  在GitHub仓库的Releases页面创建新版本，上传所有压缩包，并记录下载URL（如`https://github.com/<用户>/<仓库>/releases/download/v0.1.0/my_cli-x86_64-apple-darwin.tar.gz`）。

---

### **3. 创建Homebrew Formula**
- **新建Formula仓库**  
  在GitHub创建名为`homebrew-<项目名>`的仓库（如`homebrew-my-cli`），并在其中创建`Formula`目录。

- **编写Formula文件**  
  在`Formula`目录下创建Ruby文件（如`my_cli.rb`），内容示例：  
  ```ruby
  class MyCli < Formula
    desc "Your CLI description"
    homepage "https://github.com/<用户>/<仓库>"
    version "0.1.0"

    on_macos do
      if Hardware::CPU.intel?
        url "https://github.com/<用户>/<仓库>/releases/download/v0.1.0/my_cli-x86_64-apple-darwin.tar.gz"
        sha256 "<Intel SHA256>"
      elsif Hardware::CPU.arm?
        url "https://github.com/<用户>/<仓库>/releases/download/v0.1.0/my_cli-aarch64-apple-darwin.tar.gz"
        sha256 "<ARM SHA256>"
      end
    end

    on_linux do
      url "https://github.com/<用户>/<仓库>/releases/download/v0.1.0/my_cli-x86_64-unknown-linux-gnu.tar.gz"
      sha256 "<Linux SHA256>"
    end

    def install
      bin.install "my_cli"
    end
  end
  ```
  **注意**：需替换`url`、`sha256`和平台条件。

---

### **4. 发布与用户安装**
- **推送Formula仓库**  
  将`homebrew-my-cli`仓库推送到GitHub。

- **用户安装流程**  
  用户通过以下命令安装：  
  ```bash
  brew tap <用户>/homebrew-my-cli  # 关联Formula仓库
  brew install my-cli             # 安装CLI工具
  ```

---

### **5. 维护与更新**
- **版本更新**  
  每次发布新版本时，更新GitHub Releases的压缩包和Formula中的`url`、`sha256`及`version`字段。
  
- **多平台兼容性测试**  
  使用CI工具（如GitHub Actions）自动化编译、打包和测试，确保各平台二进制文件正常运行。

---

### **常见问题处理**
- **依赖管理**：若应用依赖系统库（如OpenSSL），需在Formula中通过`depends_on`声明。
- **路径配置**：通过`bin.install`将可执行文件链接到`/usr/local/bin`，用户可直接调用。

通过以上步骤，可实现Rust CLI应用在Homebrew上的多平台分发。具体实践可参考[CSDN博客的多平台分发教程]和[跨平台编译配置]。