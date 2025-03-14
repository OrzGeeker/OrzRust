在 Visual Studio Code 中配置 Rust 代码自动格式化，可以通过以下步骤实现：

---

### **1. 安装必要工具**
#### **1.1 安装 `rustfmt`**
Rust 的官方格式化工具是 `rustfmt`，需确保已安装：
```bash
# 安装 rustfmt（如果未包含在默认工具链中）
rustup component add rustfmt
```

#### **1.2 安装 VS Code 扩展**
- **必装扩展**：  
  - [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)  
    （提供 Rust 语言支持，默认集成格式化功能）

---

### **2. 配置自动格式化**
#### **2.1 启用保存时自动格式化**
1. **打开 VS Code 设置**：  
   - 快捷键：`Ctrl/Cmd + ,`（或通过菜单 `File > Preferences > Settings`）。
2. **搜索 `format on save`**，勾选选项：  
   ![Format on Save](https://i.imgur.com/4sE6LzM.png)

#### **2.2 手动配置（可选）**
如果默认配置不生效，在项目根目录的 `.vscode/settings.json` 或全局设置中添加：
```json
{
  "[rust]": {
    "editor.defaultFormatter": "rust-lang.rust-analyzer",
    "editor.formatOnSave": true
  }
}
```

---

### **3. 自定义格式化规则**
#### **3.1 创建 `rustfmt.toml` 文件**
在项目根目录新建 `rustfmt.toml`，自定义格式化规则，例如：
```toml
# 设置最大行宽为 100
max_width = 100
# 缩进为 2 空格
tab_spaces = 2
# 强制链式方法换行显示
chain_width = 0
```

#### **3.2 常用配置项**
| 参数                | 说明                     | 示例值       |
|---------------------|--------------------------|-------------|
| `max_width`         | 单行最大字符数           | `100`       |
| `tab_spaces`        | 缩进空格数               | `2`或`4`    |
| `chain_indent`      | 链式方法缩进风格         | `"Block"`   |
| `reorder_imports`   | 自动排序导入语句         | `true`/`false` |
| 更多规则参考：[rustfmt 文档](https://rust-lang.github.io/rustfmt/) | |

---

### **4. 验证格式化**
#### **4.1 手动触发格式化**
- **快捷键**：  
  - Windows/Linux：`Shift + Alt + F`  
  - macOS：`Shift + Option + F`  
- **右键菜单**：  
  在代码编辑器中右键选择 `Format Document`。

#### **4.2 检查输出**
如果格式化失败，查看 VS Code 的 `Output` 面板（`Ctrl/Cmd + Shift + U`），选择 `rust-analyzer` 日志，排查错误。

---

### **常见问题解决**
#### **Q1：格式化未生效**
- 确保 `rustfmt` 已安装：`rustfmt --version`。
- 检查 `rust-analyzer` 是否为默认格式化工具（避免与其他扩展冲突）。

#### **Q2：自定义规则不生效**
- 确认 `rustfmt.toml` 文件位于项目根目录。
- 重启 VS Code 或重新加载窗口（`Ctrl/Cmd + Shift + P` → `Reload Window`）。

---

通过以上配置，即可在 VS Code 中实现 Rust 代码的自动格式化，提升开发效率。