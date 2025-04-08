//AppMain 是 Makepad 中用于定义应用程序主循环的 trait。
impl AppMain for App {
    /* 这是 AppMain trait 要求实现的唯一方法。它负责处理应用程序中的所有事件，例如鼠标点击、键盘输入等。
    cx: &mut Cx: Cx 是 Makepad 的核心上下文对象，它包含了应用程序的状态和资源。
    event: &Event: Event 是一个枚举类型，表示各种不同的事件。
    self.ui.handle_event(...): 这行代码将事件传递给 ui 组件进行处理。Scope::empty() 表示一个空的事件作用域。
    */
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        /* 
        self.ui: 这表示访问 App 实例的 ui 字段。根据上下文，ui 字段是一个 WidgetRef 类型的智能指针，它指向应用程序的根 UI 组件。
            中文解释: 访问 App 实例的 ui 字段，它指向根 UI 组件。

        .handle_event(cx, event, &mut Scope::empty()): 这调用了 ui 组件的 handle_event 方法，并将 cx、event 和 &mut Scope::empty() 作为参数传递给它。
            cx: 将 Makepad 上下文对象传递给 ui 组件，以便它可以使用应用程序的状态和资源。
            event: 将发生的事件传递给 ui 组件，以便它能够处理该事件。
            &mut Scope::empty(): 这创建了一个空的 Scope 对象，并将其作为可变引用传递给 ui 组件。Scope 用于管理事件的作用域，Scope::empty() 表示一个空的事件作用域。
            中文解释: 调用根 UI 组件的 handle_event 方法来处理事件，并将上下文对象、事件对象和一个空的事件作用域传递给它。
         */
        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
}

use makepad_widgets::*;
/* 
live_design! 是 Makepad 框架提供的一个宏，它的主要作用是：
声明式 UI 定义 (Declarative UI Definition): 它允许你以一种声明式的方式来描述 UI 的结构和外观，而不是通过编写大量的命令式代码。
实时更新 (Live Reloading): 它是 Makepad 实时开发体验的核心。你在 live_design! 宏中修改 UI 定义后，运行中的应用程序会自动更新，无需重新编译。
类似 DSL (Domain-Specific Language): 它提供了一种类似领域特定语言的语法，专门用于描述 UI。
*/
live_design!(
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use crate::ui::*;//从当前项目 (crate) 的 ui 模块导入所有内容（例如：主界面组件）。
    /* 
    App = {{App}} { ... } (定义 App UI 组件):

    App =: 这定义了一个名为 App 的 UI 组件。这个 App 是整个应用程序的根 UI 组件。
    {{App}}: 双大括号 {{ }} 是 live_design! 宏的特殊语法。它表示这个 UI 组件 (App) 与 Rust 代码中定义的 App 结构体 (struct App) 之间存在直接的对应关系。这意味着，在 Rust 代码中对 App 结构体的修改，会直接影响到这里定义的 UI 组件。
    中文解释: 定义一个名为 App 的 UI 组件，它与 Rust 代码中的 App 结构体相对应。
    { ... }: 大括号内部定义了 App UI 组件的结构和属性。

    ui: <Ui> {} (定义 ui 字段):

    ui:: 这定义了 App UI 组件的一个字段，名为 ui。
    <Ui>: 尖括号 < > 表示一个类型。Ui 是在 crate::ui 模块中定义的 UI 组件类型。它很可能代表了应用程序的主要内容区域。
    中文解释: ui 字段的类型是 Ui，Ui 是在 crate::ui 模块中定义的。
    {}: 空的大括号表示使用 Ui 组件的默认配置。如果需要自定义 Ui 组件的属性，可以在这里添加配置项。
    中文解释: 使用 Ui 组件的默认配置。
    */
    App = {{App}} { // {{App}} 表示它对应于 Rust 代码中的 App 结构体
        ui: <Ui> {} //这行代码定义了 App 的一个名为 ui 的字段，它的类型是 <Ui>。<Ui> 是在 crate::ui 模块中定义的 UI 组件。{} 表示 Ui 组件的默认配置。
    }
);
#[derive(Live, LiveHook)]
struct App { //名为 App 的结构体，它代表了整个应用程序
    #[live] //属性宏表示 ui 字段是一个 Live 字段，这意味着它可以在运行时被修改和更新
    ui: WidgetRef, //ui 是 App 结构体的一个字段，它的类型是 WidgetRef。WidgetRef 是 Makepad 中用于引用 UI 组件的智能指针。
}
/* 
这是一个属性宏，用于自动为 App 结构体实现 Live 和 LiveHook trait。
Live: 这个 trait 是 Makepad 的核心，它允许 UI 在运行时进行热重载和动态更新。
LiveHook: 这个 trait 提供了在 Live 模块加载和卸载时执行代码的钩子函数。
 */
//LiveRegister trait 用于注册 Live 模块
impl LiveRegister for App {
    //这是 LiveRegister trait 要求实现的方法。它负责注册应用程序中使用的所有 Live 模块。
    fn live_register(cx: &mut Cx) {
        makepad_widgets::live_design(cx); //注册 Makepad 核心 UI 组件库
        moly_kit::live_design(cx);//注册 moly_kit 模块，可能是项目中的一个自定义库
        //注册当前项目中的其他模块，这些模块很可能包含了应用程序的特定功能和 UI 组件。
        crate::meta::live_design(cx);
        crate::bot_selector::live_design(cx);
        crate::demo_chat::live_design(cx);
        crate::ui::live_design(cx);
    }
}

app_main!(App);
