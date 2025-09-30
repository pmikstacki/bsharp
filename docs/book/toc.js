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
        this.innerHTML = '<ol class="chapter"><li class="chapter-item expanded affix "><a href="index.html">Introduction</a></li><li class="chapter-item expanded affix "><li class="part-title">Parser Architecture</li><li class="chapter-item expanded "><a href="parser/overview.html"><strong aria-hidden="true">1.</strong> Overview</a></li><li class="chapter-item expanded "><a href="parser/core-components.html"><strong aria-hidden="true">2.</strong> Core Components</a></li><li class="chapter-item expanded "><a href="parser/ast-structure.html"><strong aria-hidden="true">3.</strong> AST Structure</a></li><li class="chapter-item expanded "><a href="parser/error-handling.html"><strong aria-hidden="true">4.</strong> Error Handling</a></li><li class="chapter-item expanded affix "><li class="part-title">Parser Modules</li><li class="chapter-item expanded "><a href="parser/expressions.html"><strong aria-hidden="true">5.</strong> Expression Parsing</a></li><li class="chapter-item expanded "><a href="parser/statements.html"><strong aria-hidden="true">6.</strong> Statement Parsing</a></li><li class="chapter-item expanded "><a href="parser/declarations.html"><strong aria-hidden="true">7.</strong> Declaration Parsing</a></li><li class="chapter-item expanded "><a href="parser/types.html"><strong aria-hidden="true">8.</strong> Type System</a></li><li class="chapter-item expanded affix "><li class="part-title">Advanced Features</li><li class="chapter-item expanded "><a href="parser/navigation.html"><strong aria-hidden="true">9.</strong> AST Navigation</a></li><li class="chapter-item expanded "><a href="parser/comments.html"><strong aria-hidden="true">10.</strong> Comment Parsing</a></li><li class="chapter-item expanded "><a href="parser/preprocessor.html"><strong aria-hidden="true">11.</strong> Preprocessor Directives</a></li><li class="chapter-item expanded affix "><li class="part-title">Analysis Framework</li><li class="chapter-item expanded "><a href="analysis/overview.html"><strong aria-hidden="true">12.</strong> Analysis Overview</a></li><li class="chapter-item expanded "><a href="analysis/pipeline.html"><strong aria-hidden="true">13.</strong> Analysis Pipeline</a></li><li class="chapter-item expanded "><a href="analysis/traversal-guide.html"><strong aria-hidden="true">14.</strong> Traversal Guide</a></li><li class="chapter-item expanded "><a href="analysis/control-flow.html"><strong aria-hidden="true">15.</strong> Control Flow Analysis</a></li><li class="chapter-item expanded "><a href="analysis/dependencies.html"><strong aria-hidden="true">16.</strong> Dependency Analysis</a></li><li class="chapter-item expanded "><a href="analysis/metrics.html"><strong aria-hidden="true">17.</strong> Metrics Collection</a></li><li class="chapter-item expanded "><a href="analysis/types.html"><strong aria-hidden="true">18.</strong> Type Analysis</a></li><li class="chapter-item expanded "><a href="analysis/quality.html"><strong aria-hidden="true">19.</strong> Code Quality</a></li><li class="chapter-item expanded affix "><li class="part-title">CLI Tools</li><li class="chapter-item expanded "><a href="cli/overview.html"><strong aria-hidden="true">20.</strong> Command Line Interface</a></li><li class="chapter-item expanded "><a href="cli/parse.html"><strong aria-hidden="true">21.</strong> Parse Command</a></li><li class="chapter-item expanded "><a href="cli/tree.html"><strong aria-hidden="true">22.</strong> Tree Visualization</a></li><li class="chapter-item expanded "><a href="cli/compile.html"><strong aria-hidden="true">23.</strong> Compilation</a></li><li class="chapter-item expanded "><a href="cli/analyze.html"><strong aria-hidden="true">24.</strong> Analysis Command</a></li><li class="chapter-item expanded affix "><li class="part-title">Workspace</li><li class="chapter-item expanded "><a href="workspace/overview.html"><strong aria-hidden="true">25.</strong> Workspace Loading</a></li><li class="chapter-item expanded affix "><li class="part-title">Development</li><li class="chapter-item expanded "><a href="development/contributing.html"><strong aria-hidden="true">26.</strong> Contributing</a></li><li class="chapter-item expanded "><a href="development/testing.html"><strong aria-hidden="true">27.</strong> Testing</a></li><li class="chapter-item expanded "><a href="development/architecture.html"><strong aria-hidden="true">28.</strong> Architecture Decisions</a></li></ol>';
        // Set the current, active page, and reveal it if it's hidden
        let current_page = document.location.href.toString().split("#")[0].split("?")[0];
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
