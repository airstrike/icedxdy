use icedxdy::Component;
use icedxdy::widget::alert;

use crate::icon;
use super::Message;

pub fn view<'a>() -> Component<'a, Message> {
    alert::alert()
        .icon(icon::circle_check_big())
        .title("Account updated successfully")
        .description("Your profile information has been saved. Changes will be reflected immediately.")
        .into()
}
