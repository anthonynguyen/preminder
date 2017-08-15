# `preminder`

preminder is a Rust program that creates reminders about Github pull requests.

## Configuration

Configuration is done via JSON/YAML/TOML. A sample YAML configuration file could
look like this:

```yaml
github:
  token: secret!
  host: github.organization.org
  subjects:
    - user1
    - user2

recent: 6h
stale: 12h

outputs:
  - type: stdout
    disable: true
```

## Outputs

Every output has an optional `disable` boolean option to disable the output.

### STDOUT

The `stdout` output takes no configuration.

### Hipchat

`hipchat` output configuration options:

+ `url` - address to Hipchat instance
+ `room` - room number to send messages to
+ `token` - API token with at least a scope of `send_notification`
+ `from` - *optional* string to include next to the username
+ `colour` - *optional* colour to use as message background
+ `notify` - *optional* whether or not a desktop notification should be
  triggered
+ `template` - *optional* custom Handlebars template to format the message
+ `max_results` - *optional* limit results for each of the three lists

Template variables:

+ `now` - current time (in the following format: `Jan 12,  8:52pm`)
+ `recent_period` - human-friendly formatting of the `recent` option
+ `stale_period` - human-friendly formatting of the `stale` option
+ `num_total` - total open PRs
+ `num_opened` - number of PRs recently opened
+ `num_updated` - number of PRs recently updated
+ `num_stale` - number of old PRs
+ `opened` - list of recently opened PRs
+ `updated` - list of recently updated PRs
+ `stale` - list of old PRs

The structure of a PR object, as found in the `opened`, `updated`, or `stale`
lists can be found in `src/types.rs`.

### Email

`email` output configuration options:

+ `smtp_server` - address of the SMTP server to use for sending emails
+ `smtp_port` - *optional* port of the SMTP server (default: `25`)
+ `smtp_username` - *optional* SMTP username
+ `smtp_password` - *optional* SMTP password
+ `from_address` - email address to use in the `from:` field
+ `from_name` - *optional* name to use in the `from:` field
+ `to_address` - email address to send reminder to
+ `subject_template` - *optional* custom Handlebars template to format the
   email's subject
+ `body_template` - *optional* custom Handlebars template to format the
   email's body

**Note:** `subject_template` and `body_template` use the same variables as
described in the Hipchat output.

## License

preminder is licensed under the MIT license. Please see the `LICENSE` file for
more details.
