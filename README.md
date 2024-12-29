# ublacklist-filter

[日本語版](./README-JA.md)

This is a filter for uBlacklist that excludes search results from specific domains from various search engines, including Google etc...

To subscribe, you need to install uBlacklist.

- Firefox: [uBlacklist](https://addons.mozilla.org/ja/firefox/addon/ublacklist/)
  - You can also subscribe if you are using Firefox for Android.
- Safari: [uBlacklist for Safari](https://apps.apple.com/jp/app/ublacklist-for-safari/id1547912640)

> [!WARNING]
>
> This list has only been tested with uBlacklist on Firefox. Maintenance will only be conducted for Firefox and Safari.

## How to Use

After installing uBlacklist, add the following URL to the subscription list from the settings page.

```
https://raw.githubusercontent.com/m1sk9/ublacklist-filter/main/build/ublacklist-filter.txt
```

For detailed subscription methods, please refer to the [official documentation](https://iorate.github.io/ublacklist/ja/docs/advanced-features#subscription).

## Blocking Criteria

> [!IMPORTANT]
>
> To site administrators who are trying to send an issue to unblock their site from this filter:
>
> **Do not send it**. I created this list to exclude your site from search results based on my own will.
> Even if you send an issue to unblock, I will either close it or ignore it. Please do not waste your effort.

- There are no specific criteria for blocking in this filter.
- As mentioned above, this filter is created to exclude specific domains from search results based on my own will.
  - The reason for publishing it on GitHub is to allow other users to use it for similar purposes.
- However, the following sites will be blocked without exception:
  - Programming schools: Blog articles written by graduates are of no use.
  - Sites that machine translate GitHub or Stack Overflow: What is the purpose of these sites?

If you want to use uBlacklist more extensively, it is recommended to subscribe to filters created by other users as well. **This filter is quite extreme.**

For more details, please refer to [here](#recommended-filters).

## Adding/Removing Filters

**As mentioned above, I will not respond to requests for removal.**

- For adding filters, issues or pull requests are accepted.
  - However, whether to accept them or not is at my discretion.
  - Submitting a pull request requires a Rust environment, so it is not recommended.
- Rules are written in `rule.toml`.
  - This filter is for **domains**. Specify the domain to block the entire site.
  - Write the domain in `domain` and the comment in `comment`.

```toml
[[rule]]
domain = "example.com"
comment = "This is an example rule."
```

- Once the rule is added, run `make build` to generate `build/ublacklist-filter.txt`.
  - You can run tests with `make test`.

## Recommended Filters for Subscription

A list of subscribable filters can also be found in the [official documentation](https://iorate.github.io/ublacklist/ja/subscriptions). Here are just a few examples.

My list [`👶 ublacklist`](https://github.com/stars/m1sk9/lists/ublacklist) includes the uBlacklist filters that I actually use.

- [ncaq](https://github.com/ncaq)'s [uBlacklistRule](https://github.com/ncaq/uBlacklistRule)
- [108EAA0A](https://github.com/108EAA0A)'s [ublacklist-programming-school](https://github.com/108EAA0A/ublacklist-programming-school)
- [arosh](https://github.com/arosh)'s [ublacklist-stackoverflow-translation](https://github.com/arosh/ublacklist-stackoverflow-translation)
