// Populate the sidebar
//
// This is a script, and not included directly in the page, to control the total size of the book.
// The TOC contains an entry for each page, so if each page includes a copy of the TOC,
// the total size of the page becomes O(n**2).
class MDBookSidebarScrollbox extends HTMLElement {
    constructor() {
        super();
    }
    connectedCallback() {
        this.innerHTML = '<ol class="chapter"><li class="chapter-item expanded affix "><a href="index.html">Introduction</a></li><li class="chapter-item expanded "><a href="01-background/index.html"><strong aria-hidden="true">1.</strong> Background</a></li><li class="chapter-item expanded "><a href="02-requirements/index.html"><strong aria-hidden="true">2.</strong> Hardware/knowledge requirements</a></li><li class="chapter-item expanded "><a href="03-setup/index.html"><strong aria-hidden="true">3.</strong> Setting up a development environment</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="03-setup/linux.html"><strong aria-hidden="true">3.1.</strong> Linux</a></li><li class="chapter-item expanded "><a href="03-setup/windows.html"><strong aria-hidden="true">3.2.</strong> Windows</a></li><li class="chapter-item expanded "><a href="03-setup/macos.html"><strong aria-hidden="true">3.3.</strong> macOS</a></li><li class="chapter-item expanded "><a href="03-setup/verify.html"><strong aria-hidden="true">3.4.</strong> Verify the installation</a></li></ol></li><li class="chapter-item expanded "><a href="04-meet-your-hardware/index.html"><strong aria-hidden="true">4.</strong> Meet your hardware</a></li><li class="chapter-item expanded "><a href="05-led-roulette/index.html"><strong aria-hidden="true">5.</strong> LED roulette</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="05-led-roulette/build-it.html"><strong aria-hidden="true">5.1.</strong> Build it</a></li><li class="chapter-item expanded "><a href="05-led-roulette/flash-it.html"><strong aria-hidden="true">5.2.</strong> Flash it</a></li><li class="chapter-item expanded "><a href="05-led-roulette/debug-it.html"><strong aria-hidden="true">5.3.</strong> Debug it</a></li><li class="chapter-item expanded "><a href="05-led-roulette/the-led-and-delay-abstractions.html"><strong aria-hidden="true">5.4.</strong> The led and delay abstractions</a></li><li class="chapter-item expanded "><a href="05-led-roulette/the-challenge.html"><strong aria-hidden="true">5.5.</strong> The challenge</a></li><li class="chapter-item expanded "><a href="05-led-roulette/my-solution.html"><strong aria-hidden="true">5.6.</strong> My solution</a></li></ol></li><li class="chapter-item expanded "><a href="06-hello-world/index.html"><strong aria-hidden="true">6.</strong> Hello, world!</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="06-hello-world/panic.html"><strong aria-hidden="true">6.1.</strong> panic!</a></li></ol></li><li class="chapter-item expanded "><a href="07-registers/index.html"><strong aria-hidden="true">7.</strong> Registers</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="07-registers/rtrm.html"><strong aria-hidden="true">7.1.</strong> RTRM</a></li><li class="chapter-item expanded "><a href="07-registers/optimization.html"><strong aria-hidden="true">7.2.</strong> (mis)Optimization</a></li><li class="chapter-item expanded "><a href="07-registers/bad-address.html"><strong aria-hidden="true">7.3.</strong> 0xBAAAAAAD address</a></li><li class="chapter-item expanded "><a href="07-registers/spooky-action-at-a-distance.html"><strong aria-hidden="true">7.4.</strong> Spooky action at a distance</a></li><li class="chapter-item expanded "><a href="07-registers/type-safe-manipulation.html"><strong aria-hidden="true">7.5.</strong> Type safe manipulation</a></li></ol></li><li class="chapter-item expanded "><a href="08-leds-again/index.html"><strong aria-hidden="true">8.</strong> LEDs, again</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="08-leds-again/power.html"><strong aria-hidden="true">8.1.</strong> Power</a></li><li class="chapter-item expanded "><a href="08-leds-again/configuration.html"><strong aria-hidden="true">8.2.</strong> Configuration</a></li><li class="chapter-item expanded "><a href="08-leds-again/the-solution.html"><strong aria-hidden="true">8.3.</strong> The solution</a></li></ol></li><li class="chapter-item expanded "><a href="09-clocks-and-timers/index.html"><strong aria-hidden="true">9.</strong> Clocks and timers</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="09-clocks-and-timers/for-loop-delays.html"><strong aria-hidden="true">9.1.</strong> for loop delays</a></li><li class="chapter-item expanded "><a href="09-clocks-and-timers/nop.html"><strong aria-hidden="true">9.2.</strong> NOP</a></li><li class="chapter-item expanded "><a href="09-clocks-and-timers/one-shot-timer.html"><strong aria-hidden="true">9.3.</strong> One-shot timer</a></li><li class="chapter-item expanded "><a href="09-clocks-and-timers/initialization.html"><strong aria-hidden="true">9.4.</strong> Initialization</a></li><li class="chapter-item expanded "><a href="09-clocks-and-timers/busy-waiting.html"><strong aria-hidden="true">9.5.</strong> Busy waiting</a></li><li class="chapter-item expanded "><a href="09-clocks-and-timers/putting-it-all-together.html"><strong aria-hidden="true">9.6.</strong> Putting it all together</a></li></ol></li><li class="chapter-item expanded "><a href="10-serial-communication/index.html"><strong aria-hidden="true">10.</strong> Serial communication</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="10-serial-communication/nix-tooling.html"><strong aria-hidden="true">10.1.</strong> *nix tooling</a></li><li class="chapter-item expanded "><a href="10-serial-communication/windows-tooling.html"><strong aria-hidden="true">10.2.</strong> Windows tooling</a></li><li class="chapter-item expanded "><a href="10-serial-communication/loopbacks.html"><strong aria-hidden="true">10.3.</strong> Loopbacks</a></li></ol></li><li class="chapter-item expanded "><a href="11-usart/index.html"><strong aria-hidden="true">11.</strong> USART</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="11-usart/send-a-single-byte.html"><strong aria-hidden="true">11.1.</strong> Send a single byte</a></li><li class="chapter-item expanded "><a href="11-usart/send-a-string.html"><strong aria-hidden="true">11.2.</strong> Send a string</a></li><li class="chapter-item expanded "><a href="11-usart/buffer-overrun.html"><strong aria-hidden="true">11.3.</strong> Buffer overrun</a></li><li class="chapter-item expanded "><a href="11-usart/uprintln.html"><strong aria-hidden="true">11.4.</strong> uprintln!</a></li><li class="chapter-item expanded "><a href="11-usart/receive-a-single-byte.html"><strong aria-hidden="true">11.5.</strong> Receive a single byte</a></li><li class="chapter-item expanded "><a href="11-usart/echo-server.html"><strong aria-hidden="true">11.6.</strong> Echo server</a></li><li class="chapter-item expanded "><a href="11-usart/reverse-a-string.html"><strong aria-hidden="true">11.7.</strong> Reverse a string</a></li><li class="chapter-item expanded "><a href="11-usart/my-solution.html"><strong aria-hidden="true">11.8.</strong> My solution</a></li></ol></li><li class="chapter-item expanded "><a href="12-bluetooth-setup/index.html"><strong aria-hidden="true">12.</strong> Bluetooth setup</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="12-bluetooth-setup/linux.html"><strong aria-hidden="true">12.1.</strong> Linux</a></li><li class="chapter-item expanded "><a href="12-bluetooth-setup/loopback.html"><strong aria-hidden="true">12.2.</strong> Loopback</a></li><li class="chapter-item expanded "><a href="12-bluetooth-setup/at-commands.html"><strong aria-hidden="true">12.3.</strong> AT commands</a></li></ol></li><li class="chapter-item expanded "><a href="13-serial-over-bluetooth/index.html"><strong aria-hidden="true">13.</strong> Serial over Bluetooth</a></li><li class="chapter-item expanded "><a href="14-i2c/index.html"><strong aria-hidden="true">14.</strong> I2C</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="14-i2c/the-general-protocol.html"><strong aria-hidden="true">14.1.</strong> The general protocol</a></li><li class="chapter-item expanded "><a href="14-i2c/lsm303dlhc.html"><strong aria-hidden="true">14.2.</strong> LSM303DLHC</a></li><li class="chapter-item expanded "><a href="14-i2c/read-a-single-register.html"><strong aria-hidden="true">14.3.</strong> Read a single register</a></li><li class="chapter-item expanded "><a href="14-i2c/the-solution.html"><strong aria-hidden="true">14.4.</strong> The solution</a></li><li class="chapter-item expanded "><a href="14-i2c/read-several-registers.html"><strong aria-hidden="true">14.5.</strong> Read several registers</a></li></ol></li><li class="chapter-item expanded "><a href="15-led-compass/index.html"><strong aria-hidden="true">15.</strong> LED compass</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="15-led-compass/take-1.html"><strong aria-hidden="true">15.1.</strong> Take 1</a></li><li class="chapter-item expanded "><a href="15-led-compass/solution-1.html"><strong aria-hidden="true">15.2.</strong> Solution 1</a></li><li class="chapter-item expanded "><a href="15-led-compass/take-2.html"><strong aria-hidden="true">15.3.</strong> Take 2</a></li><li class="chapter-item expanded "><a href="15-led-compass/solution-2.html"><strong aria-hidden="true">15.4.</strong> Solution 2</a></li><li class="chapter-item expanded "><a href="15-led-compass/magnitude.html"><strong aria-hidden="true">15.5.</strong> Magnitude</a></li><li class="chapter-item expanded "><a href="15-led-compass/calibration.html"><strong aria-hidden="true">15.6.</strong> Calibration</a></li></ol></li><li class="chapter-item expanded "><a href="16-punch-o-meter/index.html"><strong aria-hidden="true">16.</strong> Punch-o-meter</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="16-punch-o-meter/gravity-is-up.html"><strong aria-hidden="true">16.1.</strong> Gravity is up?</a></li><li class="chapter-item expanded "><a href="16-punch-o-meter/the-challenge.html"><strong aria-hidden="true">16.2.</strong> The challenge</a></li><li class="chapter-item expanded "><a href="16-punch-o-meter/my-solution.html"><strong aria-hidden="true">16.3.</strong> My solution</a></li></ol></li><li class="chapter-item expanded "><a href="explore.html"><strong aria-hidden="true">17.</strong> What&#39;s left for you to explore</a></li><li class="chapter-item expanded affix "><li class="spacer"></li><li class="chapter-item expanded affix "><a href="appendix/1-general-troubleshooting/index.html">General troubleshooting</a></li><li class="chapter-item expanded affix "><a href="appendix/2-how-to-use-gdb/index.html">How to use GDB</a></li><li class="chapter-item expanded affix "><li class="spacer"></li></ol>';
        // Set the current, active page, and reveal it if it's hidden
        let current_page = document.location.href.toString().split("#")[0];
        if (current_page.endsWith("/")) {
            current_page += "index.html";
        }
        var links = Array.prototype.slice.call(this.querySelectorAll("a"));
        var l = links.length;
        for (var i = 0; i < l; ++i) {
            var link = links[i];
            var href = link.getAttribute("href");
            if (href && !href.startsWith("#") && !/^(?:[a-z+]+:)?\/\//.test(href)) {
                link.href = path_to_root + href;
            }
            // The "index" page is supposed to alias the first chapter in the book.
            if (link.href === current_page || (i === 0 && path_to_root === "" && current_page.endsWith("/index.html"))) {
                link.classList.add("active");
                var parent = link.parentElement;
                if (parent && parent.classList.contains("chapter-item")) {
                    parent.classList.add("expanded");
                }
                while (parent) {
                    if (parent.tagName === "LI" && parent.previousElementSibling) {
                        if (parent.previousElementSibling.classList.contains("chapter-item")) {
                            parent.previousElementSibling.classList.add("expanded");
                        }
                    }
                    parent = parent.parentElement;
                }
            }
        }
        // Track and set sidebar scroll position
        this.addEventListener('click', function(e) {
            if (e.target.tagName === 'A') {
                sessionStorage.setItem('sidebar-scroll', this.scrollTop);
            }
        }, { passive: true });
        var sidebarScrollTop = sessionStorage.getItem('sidebar-scroll');
        sessionStorage.removeItem('sidebar-scroll');
        if (sidebarScrollTop) {
            // preserve sidebar scroll position when navigating via links within sidebar
            this.scrollTop = sidebarScrollTop;
        } else {
            // scroll sidebar to current active section when navigating via "next/previous chapter" buttons
            var activeSection = document.querySelector('#sidebar .active');
            if (activeSection) {
                activeSection.scrollIntoView({ block: 'center' });
            }
        }
        // Toggle buttons
        var sidebarAnchorToggles = document.querySelectorAll('#sidebar a.toggle');
        function toggleSection(ev) {
            ev.currentTarget.parentElement.classList.toggle('expanded');
        }
        Array.from(sidebarAnchorToggles).forEach(function (el) {
            el.addEventListener('click', toggleSection);
        });
    }
}
window.customElements.define("mdbook-sidebar-scrollbox", MDBookSidebarScrollbox);
