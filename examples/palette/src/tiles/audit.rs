//! Preview-02 source-card audit.
//!
//! Source: `.nova/apps/v4/registry/bases/radix/blocks/preview-02`.

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Confirmed,
    ApiGap,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub struct Tile {
    pub source: &'static str,
    pub status: Status,
    pub gaps: &'static [&'static str],
}

#[allow(dead_code)]
pub const GLOBAL_GAPS: &[&str] = &[];

#[allow(dead_code)]
pub const PREVIEW_02: &[Tile] = &[
    Tile {
        source: "contribution-history.tsx",
        status: Status::ApiGap,
        gaps: &["ChartContainer", "BarChart"],
    },
    Tile {
        source: "empty-distribute-track.tsx",
        status: Status::Confirmed,
        gaps: &[],
    },
    Tile {
        source: "qr-connect.tsx",
        status: Status::Confirmed,
        gaps: &[],
    },
    Tile {
        source: "dividend-income.tsx",
        status: Status::ApiGap,
        gaps: &["ChartContainer", "BarChart"],
    },
    Tile {
        source: "index-investing.tsx",
        status: Status::Confirmed,
        gaps: &[],
    },
    Tile {
        source: "syncing-state.tsx",
        status: Status::Confirmed,
        gaps: &[],
    },
    Tile {
        source: "payout-threshold.tsx",
        status: Status::ApiGap,
        gaps: &["Select", "Textarea"],
    },
    Tile {
        source: "claimable-balance.tsx",
        status: Status::Confirmed,
        gaps: &[],
    },
    Tile {
        source: "preferences.tsx",
        status: Status::ApiGap,
        gaps: &["Select", "FieldGroup", "FieldSeparator"],
    },
    Tile {
        source: "savings-progress.tsx",
        status: Status::ApiGap,
        gaps: &["ChartContainer", "PieChart"],
    },
    Tile {
        source: "kitchen-island.tsx",
        status: Status::ApiGap,
        gaps: &["ToggleGroup", "ItemMedia"],
    },
    Tile {
        source: "savings-targets.tsx",
        status: Status::ApiGap,
        gaps: &["InputGroup", "NativeSelect", "FieldGroup"],
    },
    Tile {
        source: "recent-transactions.tsx",
        status: Status::ApiGap,
        gaps: &["Table", "DropdownMenu"],
    },
    Tile {
        source: "sidebar-nav.tsx",
        status: Status::Confirmed,
        gaps: &[],
    },
    Tile {
        source: "faq.tsx",
        status: Status::ApiGap,
        gaps: &["Tabs"],
    },
    Tile {
        source: "payments.tsx",
        status: Status::ApiGap,
        gaps: &["DropdownMenu", "ItemMedia"],
    },
    Tile {
        source: "front-door.tsx",
        status: Status::ApiGap,
        gaps: &["Pattern background media"],
    },
    Tile {
        source: "release-catalog.tsx",
        status: Status::ApiGap,
        gaps: &["InputGroup", "ToggleGroup", "ItemMedia"],
    },
    Tile {
        source: "account-access.tsx",
        status: Status::ApiGap,
        gaps: &["Input secure/password builder", "ItemMedia"],
    },
    Tile {
        source: "card-overview.tsx",
        status: Status::ApiGap,
        gaps: &["ChartContainer", "BarChart"],
    },
    Tile {
        source: "transfer-funds.tsx",
        status: Status::ApiGap,
        gaps: &["InputGroup", "Select"],
    },
    Tile {
        source: "cover-art.tsx",
        status: Status::ApiGap,
        gaps: &["File input"],
    },
    Tile {
        source: "loading-card.tsx",
        status: Status::ApiGap,
        gaps: &["Skeleton"],
    },
    Tile {
        source: "receiving-method.tsx",
        status: Status::ApiGap,
        gaps: &[
            "RadioGroup",
            "FieldSet",
            "FieldLegend",
            "FieldContent",
            "FieldTitle",
        ],
    },
    Tile {
        source: "power-usage.tsx",
        status: Status::ApiGap,
        gaps: &["ChartContainer", "BarChart"],
    },
    Tile {
        source: "empty-connect-bank.tsx",
        status: Status::Confirmed,
        gaps: &[],
    },
    Tile {
        source: "upcoming-payments.tsx",
        status: Status::ApiGap,
        gaps: &["Calendar"],
    },
    Tile {
        source: "roller-shades.tsx",
        status: Status::ApiGap,
        gaps: &["ToggleGroup"],
    },
    Tile {
        source: "stock-performance.tsx",
        status: Status::ApiGap,
        gaps: &["Combobox", "ChartContainer", "AreaChart"],
    },
    Tile {
        source: "empty-explore-catalog.tsx",
        status: Status::Confirmed,
        gaps: &[],
    },
    Tile {
        source: "new-milestone.tsx",
        status: Status::Confirmed,
        gaps: &[],
    },
    Tile {
        source: "social-links.tsx",
        status: Status::ApiGap,
        gaps: &["InputGroup"],
    },
    Tile {
        source: "notification-settings.tsx",
        status: Status::Confirmed,
        gaps: &[],
    },
];
