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
        this.innerHTML = '<ol class="chapter"><li class="chapter-item "><a href="01_introduction.html"><strong aria-hidden="true">1.</strong> Introduction</a><a class="toggle"><div>❱</div></a></li><li><ol class="section"><li class="chapter-item "><a href="01a_requirements.html"><strong aria-hidden="true">1.1.</strong> Requirements</a></li><li class="chapter-item "><a href="01b_installation.html"><strong aria-hidden="true">1.2.</strong> Installation</a></li><li class="chapter-item "><a href="01c_before_you_start.html"><strong aria-hidden="true">1.3.</strong> Before you start</a></li></ol></li><li class="chapter-item "><a href="02_example_walkthrough.html"><strong aria-hidden="true">2.</strong> Example walkthrough</a><a class="toggle"><div>❱</div></a></li><li><ol class="section"><li class="chapter-item "><a href="02a_step_1of3.html"><strong aria-hidden="true">2.1.</strong> 1/3 Generate an ELAN-file</a></li><li class="chapter-item "><a href="02b_step_2of3.html"><strong aria-hidden="true">2.2.</strong> 2/3 Annotate events</a></li><li class="chapter-item "><a href="02c_step_3of3.html"><strong aria-hidden="true">2.3.</strong> 3/3 Generate KML/GeoJSON</a></li></ol></li><li class="chapter-item "><a href="03_commands.html"><strong aria-hidden="true">3.</strong> Commands</a><a class="toggle"><div>❱</div></a></li><li><ol class="section"><li class="chapter-item "><a href="03a_cam2eaf.html"><strong aria-hidden="true">3.1.</strong> cam2eaf</a></li><li class="chapter-item "><a href="03b_eaf2geo.html"><strong aria-hidden="true">3.2.</strong> eaf2geo</a><a class="toggle"><div>❱</div></a></li><li><ol class="section"><li class="chapter-item "><a href="03ba_geoshape.html"><strong aria-hidden="true">3.2.1.</strong> The geoshape option</a></li><li class="chapter-item "><a href="03bb_cdata.html"><strong aria-hidden="true">3.2.2.</strong> The cdata option (KML)</a></li></ol></li><li class="chapter-item "><a href="03c_locate.html"><strong aria-hidden="true">3.3.</strong> locate</a></li><li class="chapter-item "><a href="03d_inspect.html"><strong aria-hidden="true">3.4.</strong> inspect</a><a class="toggle"><div>❱</div></a></li><li><ol class="section"><li class="chapter-item "><a href="03da_inspecting_data.html"><strong aria-hidden="true">3.4.1.</strong> Inspecting data</a></li></ol></li><li class="chapter-item "><a href="03e_plot.html"><strong aria-hidden="true">3.5.</strong> plot</a></li><li class="chapter-item "><a href="03f_manual.html"><strong aria-hidden="true">3.6.</strong> manual</a></li></ol></li><li class="chapter-item "><a href="04_appendix.html"><strong aria-hidden="true">4.</strong> Appendix</a><a class="toggle"><div>❱</div></a></li><li><ol class="section"><li class="chapter-item "><a href="04a_formats.html"><strong aria-hidden="true">4.1.</strong> Formats</a></li><li class="chapter-item "><a href="04b_gopro.html"><strong aria-hidden="true">4.2.</strong> GoPro</a></li><li class="chapter-item "><a href="04c_virb.html"><strong aria-hidden="true">4.3.</strong> VIRB</a></li><li class="chapter-item "><a href="04d_ffmpeg.html"><strong aria-hidden="true">4.4.</strong> FFmpeg</a></li><li class="chapter-item "><a href="04e_elan.html"><strong aria-hidden="true">4.5.</strong> ELAN</a></li><li class="chapter-item "><a href="04f_fit_gpmf_eaf_libraries.html"><strong aria-hidden="true">4.6.</strong> FIT, GPMF, EAF libraries</a></li></ol></li><li class="chapter-item "><a href="06_references.html"><strong aria-hidden="true">5.</strong> References</a></li></ol>';
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
