use makepad_widgets::*;
use moly_kit::{BotId, ChatWidgetRefExt, EntityId, Message};

use crate::demo_chat::DemoChatWidgetExt;

live_design!(
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::demo_chat::*;

    pub Ui = {{Ui}} <Window> {
        align: {x: 0.5, y: 0.5}
        pass: { clear_color: #fff }

        // caption_bar = {
        //     caption_label = {
        //         // remove the default label
        //         label = <Label> {}
        //         <View> {
        //             width: Fill,
        //             align: {x: 0.5, y: 0.5},
        //             <Label> {
        //                 text: "moly-mini"
        //                 draw_text: {
        //                     color: #000
        //                 }
        //             }
        //         }
        //     }

        //     visible: true,
        // }

        body = <View> {
            // chat_1 = <DemoChat> {}
            chat_2 = <DemoChat> {}
        }
    }
);

#[derive(Live, Widget)]
pub struct Ui {
    #[deref]
    deref: Window,
}

impl Widget for Ui {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.deref.draw_walk(cx, scope, walk)
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.deref.handle_event(cx, event, scope);

        if let Event::Startup = event {
            let bot_id = BotId::from("idk");

            let messages = std::iter::repeat([
                Message {
                    is_writing: false,
                    body: "Hello".to_string(),
                    from: EntityId::User,
                    citations: vec![],
                },
                Message {
                    is_writing: false,
                    body: "World".to_string(),
                    from: EntityId::Bot(bot_id),
                    citations: vec!["https://github.com/ZhangHanDong/url-preview/issues/2".to_string()],
                },
            ])
            .take(1)
            .flatten()
            .collect();

            self.demo_chat(id!(chat_2))
                .chat(id!(chat))
                .borrow()
                .unwrap()
                .messages_ref()
                .borrow_mut()
                .unwrap()
                .messages = messages;
        }
    }
}

impl LiveHook for Ui {}
