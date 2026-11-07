(function () {
  if (window.top !== window) return;
  if (window.__rustBrowserBar) return;
  window.__rustBrowserBar = true;

  var SEARCH = "https://duckduckgo.com/?q=";

  function resolve(raw) {
    var v = (raw || "").trim();
    if (!v) return null;
    if (/^https?:\/\//i.test(v)) return v;
    if (!/\s/.test(v) && v.indexOf(".") > 0) return "https://" + v;
    return SEARCH + encodeURIComponent(v);
  }

  function btn(label, onClick, extra) {
    var b = document.createElement("button");
    b.textContent = label;
    b.type = "button";
    b.style.cssText =
      "padding:0 14px;height:36px;border:none;border-radius:6px;" +
      "background:#0f766e;color:#fff;font:600 15px sans-serif;" +
      "cursor:pointer;flex:none;" + (extra || "");
    b.addEventListener("click", onClick);
    return b;
  }

  function build() {
    if (!document.body || document.getElementById("rust-browser-bar")) return;

    var host = document.createElement("div");
    host.id = "rust-browser-bar";
    host.style.cssText =
      "position:fixed;top:0;left:0;right:0;z-index:2147483647;" +
      "box-sizing:border-box;";
    var root = host.attachShadow({ mode: "open" });

    var bar = document.createElement("div");
    bar.style.cssText =
      "display:flex;gap:6px;align-items:center;box-sizing:border-box;" +
      "height:calc(48px + env(safe-area-inset-top, 0px));" +
      "padding:env(safe-area-inset-top, 0px) 6px 0 6px;" +
      "background:#222;border-bottom:1px solid #444;";

    var input = document.createElement("input");
    input.type = "text";
    input.value = location.href;
    input.placeholder = "Adresse ou recherche";
    input.autocapitalize = "off";
    input.spellcheck = false;
    input.setAttribute("enterkeyhint", "go");
    input.style.cssText =
      "flex:1;min-width:0;height:36px;padding:0 10px;box-sizing:border-box;" +
      "border:1px solid #444;border-radius:6px;background:#333;color:#fff;" +
      "font:16px sans-serif;";
    input.addEventListener("focus", function () { input.select(); });
    input.addEventListener("keydown", function (e) {
      if (e.key === "Enter") {
        var u = resolve(input.value);
        if (u) location.href = u;
      }
    });

    bar.appendChild(btn("<", function () { history.back(); }));
    bar.appendChild(btn(">", function () { history.forward(); }));
    bar.appendChild(input);
    bar.appendChild(btn("Aller", function () {
      var u = resolve(input.value);
      if (u) location.href = u;
    }));

    root.appendChild(bar);
    document.body.appendChild(host);

    document.documentElement.style.setProperty(
      "margin-top",
      "calc(48px + env(safe-area-inset-top, 0px))",
      "important"
    );
  }

  if (document.body) build();
  else document.addEventListener("DOMContentLoaded", build);
})();
