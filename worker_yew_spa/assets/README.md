# Static Assets

All filed in this directory are copied to `dist` ( `trunk`'s output directory ) by `<link data-trunk rel="copy-dir" href="assets">` in `index.html`.

In production, thay are served by `--assets dist` of `deploy` script :

```json
"deploy": "trunk build --release && wrangler deploy --assets dist"
```
