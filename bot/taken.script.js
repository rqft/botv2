const el = (selector, ...args) => {
    var attrs = (args[0] && typeof args[0] === 'object' && !Array.isArray(args[0]) && !(args[0] instanceof HTMLElement)) ? args.shift() : {};
  
    let classes = selector.split(".");
    if (classes.length > 0) selector = classes.shift();
    if (classes.length) attrs.className = classes.join(" ")
  
    let id = selector.split("#");
    if (id.length > 0) selector = id.shift();
    if (id.length) attrs.id = id[0];
  
    var node = document.createElement(selector.length > 0 ? selector : "div");
    for (let prop in attrs) {
      if (attrs.hasOwnProperty(prop) && attrs[prop] != undefined) {
        if (prop.indexOf("data-") == 0) {
          let dataProp = prop.substring(5).replace(/-([a-z])/g, function (g) { return g[1].toUpperCase(); });
          node.dataset[dataProp] = attrs[prop];
        } else {
          node[prop] = attrs[prop];
        }
      }
    }
  
    const append = (child) => {
      if (Array.isArray(child)) return child.forEach(append);
      if (typeof child == "string") child = document.createTextNode(child);
      if (child) node.appendChild(child);
    };
    args.forEach(append);
  
    return node;
  };
  window.el = el;
  
  const codePointToText = (codePoint) => {
    let cps = codePoint.split("-").map(hex => parseInt(hex, 16));
    let emoji = String.fromCodePoint(...cps);
    return emoji;
  }
  const emojiUrl = (codePoint) => {
    let cp = codePoint.split("-").filter(x => x !== "fe0f").map(s => s.padStart(4, "0")).join("_");
    return `https://raw.githubusercontent.com/googlefonts/noto-emoji/main/png/128/emoji_u${cp}.png`;
    return `https://raw.githubusercontent.com/googlefonts/noto-emoji/main/svg/emoji_u${cp}.svg`
  }
  
  const mixmojiUrl = (r, c) => {
    let padZeros = r < 20220500; // Revisions before 0522 had preceding zeros
    c[0] = c[0].split(/-/g).map(s => padZeros ? s.padStart(4, "0") : s).join("-u");
    c[1] = c[1].split(/-/g).map(s => padZeros ? s.padStart(4, "0") : s).join("-u");
    return `https://www.gstatic.com/android/keyboard/emojikitchen/${r}/u${c[0]}/u${c[0]}_u${c[1]}.png`
  }
  
  const copyToClipboard = async (e) => {
    console.log("e,", e);
    try {
      const imgURL = e.target.src;
      const data = await fetch(imgURL)
      const blob = await data.blob();
      const image = new Image();
      img.src = e.target.src;
      await navigator.clipboard.write([
        new ClipboardItem({
          [blob.type]: blob
        })
      ]);
      console.log('Fetched image copied.');
    } catch (err) {
      console.error(err.name, err.message);
    }
  }
  
  const focusEmoji = (e) => {
    selectEmoji(undefined, e.target.targetId);
    selectMixmoji(undefined, pc.name.split("_"));
  }
  
  const scrollElement = (e) => {
    e.target.onscroll = undefined;
    setTimeout(() => { e.target.onscroll = scrollElement; }, 2000);
    document.documentElement.className = e.target.id.replace("#", "");
  }
  
  const setFavicon = (url) => {
    document.getElementById("favicon").href = url;
    //document.getElementById("md-image").content = url;
  }
  
  let p1 = document.getElementById("p1")
  let p2 = document.getElementById("p2")
  let pc = document.getElementById("pc");
  
  let emojiContainer = document.getElementById("emoji-container");
  let mixmojiContainer = document.getElementById("mixmoji-container");
  p1.onclick = focusEmoji;
  p2.onclick = focusEmoji;
  
  let selectedMixmoji = undefined;
  const selectMixmoji = (e, parents) => {
    if (e) document.documentElement.className = "mixmoji-container";
    let img = e?.target || document.getElementById(parents.join("_")) || document.getElementById(parents.reverse().join("_"));
  
  
    if (!img) return;
  
    selectedMixmoji?.classList.remove("selected")
    selectedMixmoji = img;
    selectedMixmoji.classList.add("selected");
  
    parents = img?.c ? Array.from(img?.c) : undefined;
  
    let comboString = parents.map(codePointToText).join(" + ");
    console.log("Selecting Mix", img.id, comboString);
  
    pc.src = img.src;
    setFavicon(pc.src);
    document.getElementById("md-title").content = comboString;
  
    document.getElementById("preview-container").classList.add("mix")
    pc.name = parents.join("_");
    document.title = "= " + comboString;
  
  
    gtag('event', 'view_item', { 'event_label': comboString, 'event_category': 'mixmoji', 'non_interaction': !e });
  
    let p2id = (parents[0] == emoji1.id) ? parents.pop() : parents.shift();
    let p1id = parents.pop();
  
    emoji1?.classList.remove("selected");
    emoji2?.classList.remove("secondary");
    emoji1 = document.getElementById(p1id);
    emoji2 = document.getElementById(p2id);
    emoji1?.classList.add("selected");
    emoji2?.classList.add("secondary");
  
    p1.src = emoji1.src.replace("128", "512");
    p1.targetId = p1id;
    p2.src = emoji2.src.replace("128", "512");
    p2.targetId = p2id;
    p2.parentElement.classList.add("active");
  
    let url = "/kitchen/?" + img.c.map(cc => codePointToText(cc)).join("+");
    url += "=" + parseInt(img.date, 16).toString(36);
    window.history.replaceState({}, "", url);
  }
  
  let emoji1 = undefined;
  let emoji2 = undefined;
  let pinnedEmoji = undefined;
  
  let recents = localStorage.getItem("recents") ? JSON.parse(localStorage.getItem("recents")) : [];
  let favorites = localStorage.getItem("favorites") ? JSON.parse(localStorage.getItem("favorites")) : [];
  
  const clickedEmoji = (e) => {
    if (e) document.documentElement.className = "emoji-container";
    let target = e.target.closest("div");
  
    gtag('event', 'view_item', { 'event_label': codePointToText(target.id), 'event_category': 'emoji', 'non_interaction': true });
  
    if (target == pinnedEmoji) {
      console.log("unpin", pinnedEmoji.title);
      pinnedEmoji.classList.remove("pinned");
      pinnedEmoji = undefined;
    } else if (e.detail == 2) {
      console.log("pinning", target.title)
      pinnedEmoji?.classList.remove("pinned");
      pinnedEmoji = target;
      pinnedEmoji.classList.add("pinned");
      return;
    }
    selectEmoji(undefined, target.id);
  
    if (pinnedEmoji) {
      selectMixmoji(undefined, [pinnedEmoji.id, target.id]);
    }
  }
  
  const imageLoaded = (e) => { e.target.classList.add("loaded") }
  
  const selectEmoji = (e, id) => {
    let target = e?.target ?? document.getElementById(id);
    id = target.id;
    console.log("Selecting Base", id, codePointToText(id));
    document.getElementById("preview-container").classList.remove("mix")
  
    window.history.replaceState({}, "", "/kitchen/?" + codePointToText(id));
  
    emoji1?.classList.remove("selected");
    emoji2?.classList.remove("secondary");
  
    if (pinnedEmoji) {
      emoji2 = target;
      emoji1 = pinnedEmoji;
    } else {
      emoji2 = emoji1;
      emoji1 = target;
    }
  
    emoji1?.classList.add("selected");
    emoji2?.classList.add("secondary");
  
    recents = recents.filter(i => i !== id)
    recents.unshift(id);
    recents.splice(36);
    localStorage.setItem("recents", JSON.stringify(recents));
  
    document.title = " = " + codePointToText(id);
  
    setFavicon(target.src);
    p1.src = emoji1.src.replace("128", "512");;
    p2.src = emoji2?.src.replace("128", "512");;
    pc.src = "";
  
    emojiContainer.onscroll = scrollElement;
    mixmojiContainer.onscroll = scrollElement;
  
    const re = new RegExp("^.*\/" + target.id + "\/.*$", "gm");
  
    const array = [...window.pairs.matchAll(re)];
  
    let parent = document.getElementById("mixmoji-container");
    parent.classList.remove("hidden");
    parent.scrollTo(0, 0);
    parent.childNodes.forEach(child => { parent.removeChild(child) });
    let validPairs = []
    let div = el("div#mixmoji-content", { className: array.length < 20 ? "sparse content" : "content" },
      array.map(match => {
        let [d, c1, c2] = match.pop().split("/");
        let className = ["mixmoji"];
        className.push("c-" + c1);
        className.push("c-" + c2);
        let altParent = c1 == id ? c2 : c1;
        let index = recents.indexOf(altParent);
        let date = parseInt(d, 16) + 20200000;
        if (index == 0 && c1 == c2) {
          index = -1;
        }
        validPairs.push(altParent)
  
        let url = mixmojiUrl(date, [c1, c2]);
        if (index > 0 || c1 == c2) {
          className.push("featured");
        }
  
        return el("img", {
          id: [c1, c2].join("_"),
          date: d,
          className: className.join(" "), c: [c1, c2], onclick: selectMixmoji,
          style: "transition: all 0.3s " + Math.random() / 8 + "s ease-out;" + (index < 0 ? "" : "order:" + (-10 + index)),
          onload: imageLoaded,
          src: url,
          loading: "lazy"
        }, codePointToText(c1), codePointToText(c2))
      })
    )
    if (1) {
      [...document.querySelectorAll(".emoji")].forEach(el => {
        el.classList.toggle("dud", !validPairs.includes(el.id));
      })
    }
    parent.appendChild(div);
    if (1) {
      selectMixmoji(undefined, [emoji1?.id, emoji2?.id]);
    }
  }
  
  let div = el("div#emoji-content.content", {},
    window.points.map(point => {
      let dud = window.duds.includes(point);
      let url = emojiUrl(point);
      let text = codePointToText(point);
      let className = ["emoji"];
      if (dud) className.push("dud");
      if (favorites.includes("point")) className.push("favorite");
      return el("div", { id: point, title: text, src: url, className: className.join(" ") }, el("span", text),
        el("img", { onclick: clickedEmoji, onload: imageLoaded, src: url, loading: "lazy" })
      );
    })
  )
  emojiContainer.appendChild(div);
  
  let query = decodeURIComponent(location.search.substring(1));
  if (query.includes("&")) query = "";
  if (query.length) {
    let date = undefined;
    if (query.indexOf("=")) {
      [query, date] = query.split("=");
      date = parseInt(date, 36);
    }
    let components = query.split("+");
  
    components = components.map(c => Array.from(decodeURIComponent(c)).map(a => a.codePointAt(0).toString(16)).join("-"));
  
    if (components.length > 0) {
      document.documentElement.className = "mixmoji-container";
  
      selectEmoji(undefined, components[0])
      if (components.length > 1) {
        selectMixmoji(undefined, components);
      }
    }
  }
  document.body.addEventListener('touchmove', function (e) { e.preventDefault(); });
  
  if (!navigator.share) document.getElementById("share").style.display = "none"
  document.getElementById("copy").style.display = "none"
  
  about = () => {
    document.documentElement.classList.add('showAbout');
    document.documentElement.classList.remove('showMenu')
  }
  
  share = () => {
    document.documentElement.classList.remove('showMenu');
    navigator.share({
      title: document.title.replace("=", "").trim(),
      url: location.href
    })
      .catch(console.error);
  }
  
  copy = () => {
    document.documentElement.classList.remove('showMenu')
  
    var text = pc.src || location.href;
    var dummy = document.createElement("input");
    document.body.appendChild(dummy);
    dummy.value = text;
    dummy.select();
    document.execCommand("copy");
    document.body.removeChild(dummy);
  
    document.body.classList.add("copied");
    setTimeout(function () {
      document.body.classList.remove("copied");
    }, 2000);
  }
  