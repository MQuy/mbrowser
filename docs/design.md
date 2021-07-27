### Overview

![flow](https://i.imgur.com/HYGPbI4.png)

- when visiting a website, browser makes a DNS lookup and a HTTP Get request.
- the content (HTML file) is parsed. When parsers finds non-blocking requests such as images, the browser will request those requests and continue parsing (the same for CSS file), but `<script>` tags without `async`, `defer` attribute will block rendering and pause the parsing of HTML. DOM tree is constructed.
- CSS file contains rules which are converted to CSSOM.
- tranversing each node in DOM Tree (from the root) and matching rules from CSSOM to create Style tree. Tags not going to displayed are not included in the Style tree.
- creating a Layout tree with nodes with their size and position according to viewport.
- layers are created based on Layout tree and then are composited and rendered into the browser.

✍️ depend on css properties changed, browser might redo everything or parts of flow. More details https://csstriggers.com/

### Notes

- TCP Slow Start specifies the congestion window be initialized between 2 and 4 segments. [A proposal](https://datatracker.ietf.org/doc/html/rfc6928#section-2), from Google which was merged, increase the upper bound to 10 segments.
  ```
  min (10*MSS, max (2*MSS, 14600)) // ~ 14KB
  ```
