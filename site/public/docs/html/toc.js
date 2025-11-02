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
        this.innerHTML = '<ol class="chapter"><li class="chapter-item expanded affix "><a href="index.html">Introduction</a></li><li class="chapter-item expanded affix "><li class="part-title">Parser Architecture</li><li class="chapter-item expanded "><a href="parser/overview.html"><strong aria-hidden="true">1.</strong> Overview</a></li><li class="chapter-item expanded "><a href="parser/core-components.html"><strong aria-hidden="true">2.</strong> Core Components</a></li><li class="chapter-item expanded "><a href="parser/ast-structure.html"><strong aria-hidden="true">3.</strong> AST Structure</a></li><li class="chapter-item expanded "><a href="parser/error-handling.html"><strong aria-hidden="true">4.</strong> Error Handling</a></li><li class="chapter-item expanded affix "><li class="part-title">Parser Modules</li><li class="chapter-item expanded "><a href="parser/expressions.html"><strong aria-hidden="true">5.</strong> Expression Parsing</a></li><li class="chapter-item expanded "><a href="parser/statements.html"><strong aria-hidden="true">6.</strong> Statement Parsing</a></li><li class="chapter-item expanded "><a href="parser/declarations.html"><strong aria-hidden="true">7.</strong> Declaration Parsing</a></li><li class="chapter-item expanded "><a href="parser/types.html"><strong aria-hidden="true">8.</strong> Type System</a></li><li class="chapter-item expanded "><a href="parser/feature-completeness.html"><strong aria-hidden="true">9.</strong> Feature Completeness</a></li><li class="chapter-item expanded "><a href="parser/keywords-and-tokens.html"><strong aria-hidden="true">10.</strong> Keywords and Tokens</a></li><li class="chapter-item expanded affix "><li class="part-title">Advanced Features</li><li class="chapter-item expanded "><a href="parser/navigation.html"><strong aria-hidden="true">11.</strong> Query API</a></li><li class="chapter-item expanded "><a href="parser/comments.html"><strong aria-hidden="true">12.</strong> Comment Parsing</a></li><li class="chapter-item expanded "><a href="parser/preprocessor.html"><strong aria-hidden="true">13.</strong> Preprocessor Directives</a></li><li class="chapter-item expanded affix "><li class="part-title">Syntax</li><li class="chapter-item expanded "><a href="syntax/spans.html"><strong aria-hidden="true">14.</strong> Spans</a></li><li class="chapter-item expanded "><a href="syntax/traits.html"><strong aria-hidden="true">15.</strong> Traits</a></li><li class="chapter-item expanded "><a href="syntax/derive-macros.html"><strong aria-hidden="true">16.</strong> Derive Macros</a></li><li class="chapter-item expanded "><a href="syntax/formatter.html"><strong aria-hidden="true">17.</strong> Formatter and Emitters</a></li><li class="chapter-item expanded affix "><li class="part-title">Analysis Framework</li><li class="chapter-item expanded "><a href="analysis/overview.html"><strong aria-hidden="true">18.</strong> Analysis Overview</a></li><li class="chapter-item expanded "><a href="analysis/pipeline.html"><strong aria-hidden="true">19.</strong> Analysis Pipeline</a></li><li class="chapter-item expanded "><a href="analysis/traversal-guide.html"><strong aria-hidden="true">20.</strong> Traversal Guide</a></li><li class="chapter-item expanded "><a href="analysis/control-flow.html"><strong aria-hidden="true">21.</strong> Control Flow Analysis</a></li><li class="chapter-item expanded "><a href="analysis/dependencies.html"><strong aria-hidden="true">22.</strong> Dependency Analysis</a></li><li class="chapter-item expanded "><a href="analysis/metrics.html"><strong aria-hidden="true">23.</strong> Metrics Collection</a></li><li class="chapter-item expanded "><a href="analysis/types.html"><strong aria-hidden="true">24.</strong> Type Analysis</a></li><li class="chapter-item expanded "><a href="analysis/quality.html"><strong aria-hidden="true">25.</strong> Code Quality</a></li><li class="chapter-item expanded "><a href="analysis/passes-and-rules.html"><strong aria-hidden="true">26.</strong> Passes and Rules</a></li><li class="chapter-item expanded "><a href="analysis/report-schema.html"><strong aria-hidden="true">27.</strong> Report Schema</a></li><li class="chapter-item expanded "><a href="analysis/writing-a-pass.html"><strong aria-hidden="true">28.</strong> Writing a Pass</a></li><li class="chapter-item expanded "><a href="analysis/writing-a-ruleset.html"><strong aria-hidden="true">29.</strong> Writing a Ruleset</a></li><li class="chapter-item expanded affix "><li class="part-title">CLI Tools</li><li class="chapter-item expanded "><a href="cli/overview.html"><strong aria-hidden="true">30.</strong> Command Line Interface</a></li><li class="chapter-item expanded "><a href="cli/parse.html"><strong aria-hidden="true">31.</strong> Parse Command</a></li><li class="chapter-item expanded "><a href="cli/tree.html"><strong aria-hidden="true">32.</strong> Tree Visualization</a></li><li class="chapter-item expanded "><a href="cli/analyze.html"><strong aria-hidden="true">33.</strong> Analysis Command</a></li><li class="chapter-item expanded "><a href="cli/format.html"><strong aria-hidden="true">34.</strong> Format Command</a></li><li class="chapter-item expanded "><a href="cli/errors-json.html"><strong aria-hidden="true">35.</strong> Parse Errors JSON</a></li><li class="chapter-item expanded affix "><li class="part-title">Workspace</li><li class="chapter-item expanded "><a href="workspace/overview.html"><strong aria-hidden="true">36.</strong> Workspace Loading</a></li><li class="chapter-item expanded affix "><li class="part-title">Configuration</li><li class="chapter-item expanded "><a href="configuration/overview.html"><strong aria-hidden="true">37.</strong> Configuration Overview</a></li><li class="chapter-item expanded affix "><li class="part-title">Development</li><li class="chapter-item expanded "><a href="development/contributing.html"><strong aria-hidden="true">38.</strong> Contributing</a></li><li class="chapter-item expanded "><a href="development/testing.html"><strong aria-hidden="true">39.</strong> Testing</a></li><li class="chapter-item expanded "><a href="development/architecture.html"><strong aria-hidden="true">40.</strong> Architecture Decisions</a></li><li class="chapter-item expanded "><a href="development/cookbooks.html"><strong aria-hidden="true">41.</strong> Cookbooks</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="development/query-cookbook.html"><strong aria-hidden="true">41.1.</strong> Query Cookbook</a></li><li class="chapter-item expanded "><a href="development/parser-cookbook.html"><strong aria-hidden="true">41.2.</strong> Parser Cookbook</a></li></ol></li><li class="chapter-item expanded "><a href="development/writing-tests.html"><strong aria-hidden="true">42.</strong> Writing Tests</a></li><li class="chapter-item expanded "><a href="development/bsharp_tests.html"><strong aria-hidden="true">43.</strong> bsharp_tests Overview</a></li><li class="chapter-item expanded "><a href="development/extending-syntax.html"><strong aria-hidden="true">44.</strong> Extending Syntax (New Nodes)</a></li><li class="chapter-item expanded "><a href="development/writing-parsers.html"><strong aria-hidden="true">45.</strong> Writing Parsers</a></li><li class="chapter-item expanded "><a href="development/spanned-parsers.html"><strong aria-hidden="true">46.</strong> Spanned-first Parsers</a></li><li class="chapter-item expanded "><a href="development/compliance/index.html"><strong aria-hidden="true">47.</strong> Compliance</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="development/compliance/overview.html"><strong aria-hidden="true">47.1.</strong> Overview</a></li><li class="chapter-item expanded "><a href="development/compliance/compliance_guide.html"><strong aria-hidden="true">47.2.</strong> Compliance Guide</a></li><li class="chapter-item expanded "><a href="development/compliance/Generator.html"><strong aria-hidden="true">47.3.</strong> Generator</a></li></ol></li><li class="chapter-item expanded "><li class="part-title">Research &amp; Development</li><li class="chapter-item expanded "><a href="cil-runtime/index.html"><strong aria-hidden="true">48.</strong> CIL Runtime</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="cil-runtime/overview.html"><strong aria-hidden="true">48.1.</strong> Overview</a></li><li class="chapter-item expanded "><a href="cil-runtime/architecture.html"><strong aria-hidden="true">48.2.</strong> Architecture</a></li><li class="chapter-item expanded "><a href="cil-runtime/phases/index.html"><strong aria-hidden="true">48.3.</strong> Phases</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="cil-runtime/phases/00-dotscope-spike.html"><strong aria-hidden="true">48.3.1.</strong> Phase 0: Dotscope Spike</a></li><li class="chapter-item expanded "><a href="cil-runtime/phases/01-vm-mvp.html"><strong aria-hidden="true">48.3.2.</strong> Phase 1: VM MVP</a></li><li class="chapter-item expanded "><a href="cil-runtime/phases/02-control-flow-and-eh.html"><strong aria-hidden="true">48.3.3.</strong> Phase 2: Control Flow + EH</a></li><li class="chapter-item expanded "><a href="cil-runtime/phases/03-object-model-and-arrays.html"><strong aria-hidden="true">48.3.4.</strong> Phase 3: Object Model and Arrays</a></li><li class="chapter-item expanded "><a href="cil-runtime/phases/04-back-compat-polish.html"><strong aria-hidden="true">48.3.5.</strong> Phase 4: Back-Compat Polish</a></li><li class="chapter-item expanded "><a href="cil-runtime/phases/05-ast-to-il-emitter.html"><strong aria-hidden="true">48.3.6.</strong> Phase 5: AST→IL Emitter</a></li></ol></li><li class="chapter-item expanded "><a href="cil-runtime/dotscope-guide.html"><strong aria-hidden="true">48.4.</strong> dotscope Guide</a></li><li class="chapter-item expanded "><a href="cil-runtime/vm-design.html"><strong aria-hidden="true">48.5.</strong> VM Design</a></li><li class="chapter-item expanded "><a href="cil-runtime/emitter-design.html"><strong aria-hidden="true">48.6.</strong> Emitter Design</a></li><li class="chapter-item expanded "><a href="cil-runtime/testing.html"><strong aria-hidden="true">48.7.</strong> Testing &amp; Conformance</a></li><li class="chapter-item expanded "><a href="cil-runtime/roadmap.html"><strong aria-hidden="true">48.8.</strong> Roadmap</a></li><li class="chapter-item expanded "><a href="cil-runtime/open-questions.html"><strong aria-hidden="true">48.9.</strong> Open Questions</a></li><li class="chapter-item expanded "><a href="cil-runtime/glossary.html"><strong aria-hidden="true">48.10.</strong> Glossary</a></li></ol></li></ol>';
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
