// API Reference is rendered via the `accordion-api-reference.md` snippet
// at the bottom of BLOCKS. The live shadcn docs page lists the Radix
// primitives (Accordion / AccordionItem / AccordionTrigger /
// AccordionContent) but defers the prop matrix to Radix's own docs —
// the snippet captures the icedxdy-shaped builder API for `accordion(...)`
// and `accordion::item(...)` plus the parent-owned-state model.
//
// Intentional skips per CLAUDE.md: accordion-rtl (RTL not applicable to
// iced).

use iced::Widget;
use std::collections::HashSet;

use iced::Task;

use icedxdy::Theme;
use icedxdy::card;
use icedxdy::theme;
use icedxdy::widget::{accordion, container, text};

use crate::Message;
use crate::icon;
use crate::page;

pub static META: page::Meta = page::Meta {
    title: "Accordion",
    description: "A vertically stacked set of interactive headings that each reveal a section of content.",
};

pub static BLOCKS: &[page::Block] = &[
    page::Block::Example {
        id: "demo",
        title: "",
        description: None,
        snippet: "examples/browser/snippets/accordion-demo.md",
    },
    page::Block::Group {
        title: "Examples",
        children: &[
            page::Block::Example {
                id: "basic",
                title: "Basic",
                description: Some(
                    "A basic accordion that shows one item at a time. The first item is open by default.",
                ),
                snippet: "examples/browser/snippets/accordion-basic.md",
            },
            page::Block::Example {
                id: "multiple",
                title: "Multiple",
                description: Some(
                    "Use `type=\"multiple\"` to allow multiple items to be open at the same time.",
                ),
                snippet: "examples/browser/snippets/accordion-multiple.md",
            },
            page::Block::Example {
                id: "disabled",
                title: "Disabled",
                description: Some(
                    "Use the `disabled` prop on `AccordionItem` to disable individual items.",
                ),
                snippet: "examples/browser/snippets/accordion-disabled.md",
            },
            page::Block::Example {
                id: "borders",
                title: "Borders",
                description: Some(
                    "Add `border` to the `Accordion` and `border-b last:border-b-0` to the `AccordionItem` to add borders to the items.",
                ),
                snippet: "examples/browser/snippets/accordion-borders.md",
            },
            page::Block::Example {
                id: "card",
                title: "Card",
                description: Some("Wrap the `Accordion` in a `Card` component."),
                snippet: "examples/browser/snippets/accordion-card.md",
            },
        ],
    },
    page::Block::Md("examples/browser/snippets/accordion-api-reference.md"),
];

/// Stable per-example open-set state. Each block has its own open-set so
/// they animate independently (clicking an item on the demo card doesn't
/// affect what's open on the disabled card).
pub struct AccordionPage {
    state: page::State,
    demo: HashSet<String>,
    basic: HashSet<String>,
    multiple: HashSet<String>,
    disabled: HashSet<String>,
    borders: HashSet<String>,
    card: HashSet<String>,
}

impl AccordionPage {
    pub fn new(page_index: usize) -> Box<dyn page::Page> {
        // Match shadcn's `defaultValue` prop on each example.
        let demo = ["shipping"].into_iter().map(String::from).collect();
        let basic = ["item-1"].into_iter().map(String::from).collect();
        let multiple = ["notifications"].into_iter().map(String::from).collect();
        let disabled = HashSet::new();
        let borders = ["billing"].into_iter().map(String::from).collect();
        let card = ["plans"].into_iter().map(String::from).collect();

        Box::new(Self {
            state: page::State::new(page_index, BLOCKS),
            demo,
            basic,
            multiple,
            disabled,
            borders,
            card,
        })
    }

    /// Apply a single-mode + collapsible toggle to an open-set. Empty
    /// returned set models "everything closed" — single-mode collapsible
    /// allows zero open. Used by every example except the multiple one.
    fn apply_single(set: &mut HashSet<String>, id: String) {
        if set.contains(&id) {
            set.clear();
        } else {
            set.clear();
            set.insert(id);
        }
    }

    /// Apply a multiple-mode toggle: insert if absent, remove if present.
    fn apply_multiple(set: &mut HashSet<String>, id: String) {
        if !set.remove(&id) {
            set.insert(id);
        }
    }
}

/// Routing tag for `Toggled(usize, String)` so the page can dispatch the
/// click to the right per-example open-set.
const F_DEMO: usize = 0;
const F_BASIC: usize = 1;
const F_MULTIPLE: usize = 2;
const F_DISABLED: usize = 3;
const F_BORDERS: usize = 4;
const F_CARD: usize = 5;

/// Helper: build a chevron-down icon at 16px, sized to nova's `size-4`.
/// Lives here rather than in the page state because the icon is used at
/// view time only (no per-frame allocation).
fn chevron_closed<'a>() -> icedxdy::Slot<'a, Message> {
    icon::chevron_down().size(16).into()
}

/// Helper: build a chevron-up icon at 16px.
fn chevron_open<'a>() -> icedxdy::Slot<'a, Message> {
    icon::chevron_up().size(16).into()
}

/// Build the accordion preview for a single example.
///
/// `page_index` routes the message through the multiplex; `field_id`
/// is the per-example routing tag (`F_DEMO` / `F_BASIC` / ...); `mode`
/// drives the widget's internal click gate; `open` is the current
/// open-set borrowed for the duration of the view. Bundling here keeps
/// each `preview()` arm to a single `accordion(...)` call.
fn build_accordion<'a>(
    page_index: usize,
    field_id: usize,
    mode: accordion::Mode,
    open: &HashSet<String>,
    items: impl IntoIterator<Item = (&'a str, &'a str, &'a str, bool)>,
) -> icedxdy::Slot<'a, Message> {
    let items: Vec<_> = items
        .into_iter()
        .map(|(id, trigger, body, disabled)| {
            accordion::item(id.to_owned(), text::small(trigger).medium())
                .content(text::small(body).style(theme::text::Muted))
                .chevron_closed(chevron_closed())
                .chevron_open(chevron_open())
                .disabled(disabled)
        })
        .collect();

    accordion(items)
        .mode(mode)
        .open(open.iter().cloned())
        .on_toggle(move |id| Message::Page(page_index, page::Message::InputChanged(field_id, id)))
        .into()
}

impl page::Page for AccordionPage {
    fn page_index(&self) -> usize {
        self.state.page_index
    }
    fn meta(&self) -> &'static page::Meta {
        &META
    }
    fn blocks(&self) -> &'static [page::Block] {
        BLOCKS
    }
    fn state(&self) -> &page::State {
        &self.state
    }
    fn scrollable_id(&self) -> iced::widget::Id {
        self.state.scrollable_id.clone()
    }

    fn init(&self) -> Task<Message> {
        self.state.init()
    }

    fn update(&mut self, event: page::Message) -> Task<Message> {
        if let Some(task) = self.state.update(&event) {
            return task;
        }
        if let page::Message::InputChanged(field, id) = event {
            match field {
                F_DEMO => Self::apply_single(&mut self.demo, id),
                F_BASIC => Self::apply_single(&mut self.basic, id),
                F_MULTIPLE => Self::apply_multiple(&mut self.multiple, id),
                F_DISABLED => Self::apply_single(&mut self.disabled, id),
                F_BORDERS => Self::apply_single(&mut self.borders, id),
                F_CARD => Self::apply_single(&mut self.card, id),
                _ => {}
            }
        }
        Task::none()
    }

    fn reset_volatile(&mut self) {
        self.state.reset_volatile();
    }

    fn preview(&self, id: &str) -> iced::Element<'_, Message, Theme> {
        let single = accordion::Mode::Single { collapsible: true };
        let multiple = accordion::Mode::Multiple;
        let pi = self.state.page_index;

        match id {
            "demo" => container(build_accordion(
                pi,
                F_DEMO,
                single,
                &self.demo,
                [
                    (
                        "shipping",
                        "What are your shipping options?",
                        "We offer standard (5-7 days), express (2-3 days), and overnight shipping. Free shipping on international orders.",
                        false,
                    ),
                    (
                        "returns",
                        "What is your return policy?",
                        "Returns accepted within 30 days. Items must be unused and in original packaging. Refunds processed within 5-7 business days.",
                        false,
                    ),
                    (
                        "support",
                        "How can I contact customer support?",
                        "Reach us via email, live chat, or phone. We respond within 24 hours during business days.",
                        false,
                    ),
                ],
            ))
            .max_width(512.0)
            .width(iced::Fill)
            .into(),

            "basic" => container(build_accordion(
                pi,
                F_BASIC,
                single,
                &self.basic,
                [
                    (
                        "item-1",
                        "How do I reset my password?",
                        "Click on 'Forgot Password' on the login page, enter your email address, and we'll send you a link to reset your password. The link will expire in 24 hours.",
                        false,
                    ),
                    (
                        "item-2",
                        "Can I change my subscription plan?",
                        "Yes, you can upgrade or downgrade your plan at any time from your account settings. Changes will be reflected in your next billing cycle.",
                        false,
                    ),
                    (
                        "item-3",
                        "What payment methods do you accept?",
                        "We accept all major credit cards, PayPal, and bank transfers. All payments are processed securely through our payment partners.",
                        false,
                    ),
                ],
            ))
            .max_width(512.0)
            .width(iced::Fill)
            .into(),

            "multiple" => container(build_accordion(
                pi,
                F_MULTIPLE,
                multiple,
                &self.multiple,
                [
                    (
                        "notifications",
                        "Notification Settings",
                        "Manage how you receive notifications. You can enable email alerts for updates or push notifications for mobile devices.",
                        false,
                    ),
                    (
                        "privacy",
                        "Privacy & Security",
                        "Control your privacy settings and security preferences. Enable two-factor authentication, manage connected devices, review active sessions, and configure data sharing preferences. You can also download your data or delete your account.",
                        false,
                    ),
                    (
                        "billing",
                        "Billing & Subscription",
                        "View your current plan, payment history, and upcoming invoices. Update your payment method, change your subscription tier, or cancel your subscription.",
                        false,
                    ),
                ],
            ))
            .max_width(512.0)
            .width(iced::Fill)
            .into(),

            "disabled" => container(build_accordion(
                pi,
                F_DISABLED,
                single,
                &self.disabled,
                [
                    (
                        "item-1",
                        "Can I access my account history?",
                        "Yes, you can view your complete account history including all transactions, plan changes, and support tickets in the Account History section of your dashboard.",
                        false,
                    ),
                    (
                        "item-2",
                        "Premium feature information",
                        "This section contains information about premium features. Upgrade your plan to access this content.",
                        true,
                    ),
                    (
                        "item-3",
                        "How do I update my email address?",
                        "You can update your email address in your account settings. You'll receive a verification email at your new address to confirm the change.",
                        false,
                    ),
                ],
            ))
            .width(iced::Fill)
            .into(),

            "borders" => {
                // Shadcn `accordion-borders.tsx`: outer `rounded-lg border`
                // wraps the accordion and each item adds `px-4`. We supply
                // both via the wrapping container + `accordion::padding(...)`.
                let inner = build_accordion(
                    pi,
                    F_BORDERS,
                    single,
                    &self.borders,
                    [
                        (
                            "billing",
                            "How does billing work?",
                            "We offer monthly and annual subscription plans. Billing is charged at the beginning of each cycle, and you can cancel anytime. All plans include automatic backups, 24/7 support, and unlimited team members.",
                            false,
                        ),
                        (
                            "security",
                            "Is my data secure?",
                            "Yes. We use end-to-end encryption, SOC 2 Type II compliance, and regular third-party security audits. All data is encrypted at rest and in transit using industry-standard protocols.",
                            false,
                        ),
                        (
                            "integration",
                            "What integrations do you support?",
                            "We integrate with 500+ popular tools including Slack, Zapier, Salesforce, HubSpot, and more. You can also build custom integrations using our REST API and webhooks.",
                            false,
                        ),
                    ],
                );

                container(inner)
                    .max_width(512.0)
                    .width(iced::Fill)
                    .padding([0, 16])
                    .style(|theme: &Theme| {
                        use icedxdy::core::Border;
                        use icedxdy::theme::container::Style;
                        let p = theme;
                        Style {
                            background: None,
                            border: Border {
                                width: 1.0,
                                color: p.neutral.border,
                                radius: icedxdy::radius::LG.into(),
                            },
                            ..Style::default()
                        }
                    })
                    .into()
            }

            "card" => {
                // Shadcn `accordion-card.tsx`: a Card wraps the accordion.
                // CardHeader has the title + description; CardContent
                // holds the accordion itself.
                let inner = build_accordion(
                    pi,
                    F_CARD,
                    single,
                    &self.card,
                    [
                        (
                            "plans",
                            "What subscription plans do you offer?",
                            "We offer three subscription tiers: Starter ($9/month), Professional ($29/month), and Enterprise ($99/month). Each plan includes increasing storage limits, API access, priority support, and team collaboration features.",
                            false,
                        ),
                        (
                            "billing",
                            "How does billing work?",
                            "Billing occurs automatically at the start of each billing cycle. We accept all major credit cards, PayPal, and ACH transfers for enterprise customers. You'll receive an invoice via email after each payment.",
                            false,
                        ),
                        (
                            "cancel",
                            "How do I cancel my subscription?",
                            "You can cancel your subscription anytime from your account settings. There are no cancellation fees or penalties. Your access will continue until the end of your current billing period.",
                            false,
                        ),
                    ],
                );

                let header = card::header("Subscription & Billing").description(
                    "Common questions about your account, plans, payments and cancellations.",
                );
                let body = card::content(inner);
                use icedxdy::widget::column;
                let stack = column![header, body];

                card(stack).width(iced::Fill.max(384.0)).boxed()
            }

            _ => unreachable!("unknown accordion preview id: {id}"),
        }
    }
}
