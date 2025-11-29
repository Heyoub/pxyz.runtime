// PXYZ Explorer web app code
const DATA = {
  components: [
    {
      name: "Rust Compiler",
      responsibility: "Transform XML to binary",
      loc: "~1500",
      stages: ["Parsing (XML → AST)", "Lowering (AST → IR)", "Predicate Compilation", "Validation (3-layer)", "Optimization", "Emission (binary)"]
    },
    {
      name: "WASM Runtime",
      responsibility: "Execute graph.bin in sandbox",
      loc: "~600",
      features: ["Auditable execution", "Formal verifiable design", "Completely sandboxed"]
    },
    {
      name: "JavaScript Host",
      responsibility: "Provide I/O bridge",
      imports: ["io_call", "io_resolve_var", "io_is_human", "io_is_confirmed"]
    }
  ],
  node_kinds: [
    { id: 0, name: "Transform", purpose: "Validate, transform, or structure data without side effects", uses: ["Data validation", "Field mapping", "Aggregation"], flags: [] },
    { id: 1, name: "External", purpose: "Call external I/O handler via operation code", uses: ["API calls", "Database operations", "Email sending"], flags: ["HAS_SIDE_EFFECTS", "IRREVERSIBLE"] },
    { id: 2, name: "Render", purpose: "Generate HTML or user-facing output from template", uses: ["UI generation", "Form rendering", "Result display"], flags: [] },
    { id: 3, name: "Signal", purpose: "Emit signal to UI framework for client-side changes", uses: ["UI updates", "State synchronization"], flags: [] },
    { id: 4, name: "Auth", purpose: "Authorization check using predicate", uses: ["Permission checks", "Access control"], flags: ["REQUIRES_AUTH"] },
    { id: 5, name: "Terminal", purpose: "Gracefully end graph traversal with success", uses: ["Workflow completion"], flags: [] },
    { id: 6, name: "Error", purpose: "Handle errors and return error status", uses: ["Error handling", "Fallback paths"], flags: [] }
  ],
  opcodes: [
    { hex: "0x00", name: "NOOP", effect: "-", desc: "No operation" },
    { hex: "0x01", name: "PUSH_INT", effect: "→ val", desc: "Push immediate 32-bit integer" },
    { hex: "0x02", name: "PUSH_STR", effect: "→ val", desc: "Push string reference (String Pool offset)" },
    { hex: "0x03", name: "LOAD_VAR", effect: "→ val", desc: "Load variable from host context" },
    { hex: "0x04", name: "LOAD_FIELD", effect: "obj → val", desc: "Get field from object reference" },
    { hex: "0x10", name: "EQ", effect: "a, b → bool", desc: "Pop two values, push 1 if equal" },
    { hex: "0x11", name: "NEQ", effect: "a, b → bool", desc: "Pop two values, push 1 if not equal" },
    { hex: "0x12", name: "GT", effect: "a, b → bool", desc: "Pop a and b, push 1 if a > b" },
    { hex: "0x13", name: "GTE", effect: "a, b → bool", desc: "Pop a and b, push 1 if a >= b" },
    { hex: "0x14", name: "LT", effect: "a, b → bool", desc: "Pop a and b, push 1 if a < b" },
    { hex: "0x15", name: "LTE", effect: "a, b → bool", desc: "Pop a and b, push 1 if a <= b" },
    { hex: "0x20", name: "AND", effect: "a, b → bool", desc: "Logical AND of two booleans" },
    { hex: "0x21", name: "OR", effect: "a, b → bool", desc: "Logical OR of two booleans" },
    { hex: "0x22", name: "NOT", effect: "a → bool", desc: "Logical NOT of boolean" },
    { hex: "0x30", name: "CONTAINS", effect: "haystack, needle → bool", desc: "Check if string/array contains value" },
    { hex: "0x31", name: "MATCHES", effect: "str → bool", desc: "Check if string matches regex" },
    { hex: "0x32", name: "STARTS_WITH", effect: "str, prefix → bool", desc: "Check if string starts with prefix" },
    { hex: "0x33", name: "ENDS_WITH", effect: "str, suffix → bool", desc: "Check if string ends with suffix" },
    { hex: "0x40", name: "LEN", effect: "val → int", desc: "Get length of string or array" },
    { hex: "0x41", name: "GET", effect: "arr, idx → val", desc: "Get element from array at index" },
    { hex: "0x42", name: "IS_NULL", effect: "val → bool", desc: "Check if value is null/undefined" },
    { hex: "0x43", name: "IS_DEFINED", effect: "val → bool", desc: "Check if value is not null/undefined" },
    { hex: "0x44", name: "IS_CONFIRMED", effect: "val → bool", desc: "Check if entity data is confirmed" },
    { hex: "0xF0", name: "CALL_PRED", effect: "→ bool", desc: "Call another predicate by ID" },
    { hex: "0xFF", name: "RET", effect: "bool →", desc: "Return boolean and terminate" }
  ],
  io_operations: {
    entity: [
      { code: "0x0100", name: "ENTITY_CREATE", irreversible: false },
      { code: "0x0101", name: "ENTITY_READ", irreversible: false },
      { code: "0x0102", name: "ENTITY_UPDATE", irreversible: false },
      { code: "0x0103", name: "ENTITY_DELETE", irreversible: true },
      { code: "0x0104", name: "ENTITY_LIST", irreversible: false },
      { code: "0x0105", name: "ENTITY_SEARCH", irreversible: false }
    ],
    communication: [
      { code: "0x0340", name: "EMAIL_SEND", irreversible: true },
      { code: "0x0350", name: "SMS_SEND", irreversible: true },
      { code: "0x0360", name: "WEBHOOK_CALL", irreversible: true }
    ]
  },
  constraints_pragmatic: [
    { code: "PRAG001", check: "LLM → Irreversible paths require validation gate" },
    { code: "PRAG002", check: "Write operations should have error branches" },
    { code: "PRAG003", check: "Irreversible actions require human-in-the-loop" },
    { code: "PRAG004", check: "Irreversible actions require confirmed inputs" },
    { code: "PRAG005", check: "Quarantined data cannot escape to IO" }
  ],
  example_workflow: `<omar>\n  <workflow>\n    <entry p=\"approval\" x=\"request\" node=\"start\"/>\n    <nodes>\n      <node id=\"start\" kind=\"transform\" schema=\"ApprovalRequest\"/>\n      <node id=\"auth_check\" kind=\"auth\" predicate=\"is_approver\"/>\n      <node id=\"send_email\" kind=\"external\" op=\"0x0340\"/>\n      <node id=\"success\" kind=\"terminal\" status=\"200\"/>\n      <node id=\"error\" kind=\"error\" status=\"403\"/>\n    </nodes>\n    <edges>\n      <edge from=\"start\" to=\"auth_check\"/>\n      <edge from=\"auth_check\" to=\"send_email\"/>\n      <edge from=\"auth_check\" to=\"error\" weight=\"0\"/>\n      <edge from=\"send_email\" to=\"success\"/>\n    </edges>\n  </workflow>\n  <predicates>\n    <predicate id=\"is_approver\">\n      <contains left=\"$token.perms\" right=\"approve\"/>\n    </predicate>\n  </predicates>\n</omar>`
};

// Simple glossary of key terms
const GLOSSARY = [
  { term: "Node", desc: "A unit of work or control in the workflow graph." },
  { term: "Edge", desc: "A directional connection between nodes, may be conditional." },
  { term: "Predicate", desc: "Boolean condition used to control graph traversal." },
  { term: "Opcode", desc: "Bytecode instruction for the Predicate VM." },
  { term: "Irreversible", desc: "An action that cannot be easily undone (e.g., send email)." },
  { term: "External Node", desc: "A node that triggers external IO operations via op code." },
  { term: "Terminal Node", desc: "A node that ends traversal and returns a result/status." },
  { term: "WASM Runtime", desc: "Sandboxed engine that executes graph.bin workflows." },
  { term: "JS Host", desc: "Application bridge providing safe IO operations to WASM." }
];

function syntaxHighlightXML(xml) {
  return xml
    .replace(/(&lt;\/?[a-zA-Z0-9_-]+)(.*?)(\/?&gt;)/g, (m, tag, attrs, close) => {
      return `<span class='hl-tag'>${tag}</span><span class='hl-attr'>${attrs}</span><span class='hl-tag'>${close}</span>`;
    })
    .replace(/&quot;(.*?)&quot;/g, "<span class='hl-str'>&quot;$1&quot;</span>")
    .replace(/\b(0x[0-9A-Fa-f]+)\b/g, "<span class='hl-hex'>$1</span>");
}

document.addEventListener('DOMContentLoaded', () => {
  const tabs = [
    { key: "overview", label: "Architecture Overview" },
    { key: "nodes", label: "Node Kinds Reference" },
    { key: "opcodes", label: "Predicate VM Opcodes" },
    { key: "io", label: "IO Operation Codes" },
    { key: "constraints", label: "Constraint System" },
    { key: "binary", label: "Binary Format Reference" },
    { key: "workflow", label: "Example Workflow" },
  ];
  let currentTab = 'overview';
  let searchQuery = '';

  function setTab(tab) {
    currentTab = tab;
    document.querySelectorAll('.tab-btn').forEach(btn => {
      btn.classList.toggle('active', btn.dataset.tab === tab);
    });
    showTabContent(tab);
  }

  function showTabContent(tab) {
    const tc = document.getElementById('tab-content');
    if (tab === 'overview') tc.innerHTML = renderOverview();
    else if (tab === 'nodes') tc.innerHTML = renderNodes();
    else if (tab === 'opcodes') tc.innerHTML = renderOpcodes(searchQuery);
    else if (tab === 'io') tc.innerHTML = renderIOCodes(searchQuery);
    else if (tab === 'constraints') tc.innerHTML = renderConstraints();
    else if (tab === 'binary') tc.innerHTML = renderBinaryRef();
    else if (tab === 'workflow') tc.innerHTML = renderExampleWorkflow();
    else tc.innerHTML = '<p>Not found.</p>';
  }

  function renderOverview() {
    return `<h2>Architecture Overview</h2>
    <section class="mt-8">
      <div class="flex gap-16 items-center" style="flex-wrap:wrap">
        <div>
          <div class="card__body"><strong>${DATA.components[0].name}</strong>
            <div class="status status--info mb-8">${DATA.components[0].responsibility}</div>
            <ul>
              <li>LOC: ${DATA.components[0].loc}</li>
              <li>Stages:<br> <ul>${DATA.components[0].stages.map(s => `<li>${s}</li>`).join('')}</ul></li>
            </ul>
          </div>
        </div>
        <span class="hl-op" style="font-size:22px;padding:24px;">&#8594;</span>
        <div>
          <div class="card__body"><strong>${DATA.components[1].name}</strong>
            <div class="status status--info mb-8">${DATA.components[1].responsibility}</div>
            <ul>
              <li>LOC: ${DATA.components[1].loc}</li>
              <li>Features:<br><ul>${DATA.components[1].features.map(f => `<li>${f}</li>`).join('')}</ul></li>
            </ul>
          </div>
        </div>
        <span class="hl-op" style="font-size:22px;padding:24px;">&#8594;</span>
        <div>
          <div class="card__body"><strong>${DATA.components[2].name}</strong>
            <div class="status status--info mb-8">${DATA.components[2].responsibility}</div>
            <ul>
              <li>Imports:<br><ul>${DATA.components[2].imports.map(i => `<li>${i}</li>`).join('')}</ul></li>
            </ul>
          </div>
        </div>
      </div>
      <div class="mt-16 mb-8 expander" id="pipeline-expander">Show Compilation Pipeline</div>
      <div id="pipeline-section" class="expander-section hidden">
        <h4>Compilation Pipeline</h4>
        <ol>
          <li><strong>Parsing:</strong> XML to AST tree.</li>
          <li><strong>Lowering:</strong> AST to IR (numeric graph).</li>
          <li><strong>Predicate Compilation:</strong> Compile predicates to bytecode.</li>
          <li><strong>Validation:</strong> Three-layer (syntactic, semantic, pragmatic) constraints.</li>
          <li><strong>Optimization:</strong> Dead code elimination, deduplication.</li>
          <li><strong>Emission:</strong> Outputs graph.bin binary artifact.</li>
        </ol>
        <div class="mt-8" style="color:var(--color-text-secondary);font-size:13px">Design emphasizes auditability, explicit IO, and deterministic workflows.<br>See tabs for details on bytecode and validation.</div>
      </div>
    </section>`;
  }

  function renderNodes() {
    return `<h2>Node Kinds Reference</h2>
    <table class="mt-8 mb-8" style="width:100%">
      <thead><tr><th>Name</th><th>Purpose</th><th>Use Cases</th><th>Flags</th><th>XML Example</th></tr></thead>
      <tbody>
        ${DATA.node_kinds.map(n => `
          <tr>
            <td><strong>${n.name}</strong></td>
            <td>${n.purpose}</td>
            <td>${n.uses.join('<br>')}</td>
            <td>${n.flags.map(f => `<span class="badge">${f}</span>`).join('')}</td>
            <td><pre class="code-block"><code>&lt;node id=&quot;demo&quot; kind=&quot;${n.name.toLowerCase()}"${n.flags.includes('IRREVERSIBLE') ? ' op="0x0340"' : ''}/&gt;</code></pre>
              <button class="copy-btn" onclick="navigator.clipboard.writeText('<node id=\"demo\" kind=\"${n.name.toLowerCase()}\"${n.flags.includes('IRREVERSIBLE') ? ' op=\"0x0340\"' : ''}/>' )">Copy</button>
            </td>
          </tr>
        `).join('')}
      </tbody>
    </table>
    <div class="mt-8">Each node kind represents a control or work unit; outgoing edges depend on kind. <span class="tooltip">Why no cycles?<span class="tooltip-content">PXYZ requires acyclic graphs for deterministic, auditable execution.</span></span></div>`;
  }

  function renderOpcodes(search) {
    let filtered = DATA.opcodes.filter(o => !search || (o.name.toLowerCase().includes(search.toLowerCase()) || o.hex.includes(search) || o.desc.toLowerCase().includes(search.toLowerCase())));
    return `<h2>Predicate VM Opcodes</h2>
      <input type="text" id="opcode-search" class="form-control mb-8" placeholder="Filter opcodes..." value="${search || ''}" aria-label="Search opcodes">
      <table style="width:100%">
        <thead><tr><th>Opcode</th><th>Name</th><th>Stack Effect</th><th>Description</th><th></th></tr></thead>
        <tbody>
          ${filtered.map(op => `
            <tr>
              <td><span class="hl-hex">${op.hex}</span></td>
              <td>${op.name}</td>
              <td>${op.effect}</td>
              <td>${op.desc}</td>
              <td><button class="copy-btn" onclick="navigator.clipboard.writeText('${op.hex}')">Copy hex</button></td>
            </tr>
          `).join('')}
        </tbody>
      </table>
      <div class="mt-16"><strong>Bytecode Example:</strong><pre class="code-block"><code>PUSH_VAR $token.perms\nPUSH_STR approve\nCONTAINS\nRET</code></pre>
      <button class="copy-btn" onclick="navigator.clipboard.writeText('PUSH_VAR $token.perms\nPUSH_STR approve\nCONTAINS\nRET')">Copy Example</button>
      </div>
      <div class="mt-8">Predicate compilation transforms XML predicate into stack bytecode for safe runtime evaluation.</div>`;
  }

  function renderIOCodes(search) {
    let sections = Object.entries(DATA.io_operations);
    return `<h2>IO Operation Codes</h2>
      ${sections.map(([cat, ops]) => `
        <h4>${cat.charAt(0).toUpperCase() + cat.slice(1)}</h4>
        <table class="mb-8" style="width:100%">
          <thead><tr><th>Code</th><th>Name</th><th>Irreversible?</th><th>Reference</th></tr></thead>
          <tbody>
          ${ops.filter(op => !search || op.code.includes(search) || op.name.toLowerCase().includes(search.toLowerCase())).map(op => `
            <tr>
              <td><span class="hl-hex">${op.code}</span></td>
              <td>${op.name}</td>
              <td>${op.irreversible ? '<span class="badge irrevocable">Irreversible</span>' : ''}</td>
              <td>${op.irreversible ? `<span class="badge">PRAG Constraint</span>` : ''}</td>
            </tr>
          `).join('')}
          </tbody>
        </table>
      `).join('')}
      <div class="mt-8">Irreversible operations are flagged and subject to safety constraints.</div>`;
  }

  function renderConstraints() {
    return `<h2>Constraint System</h2>
    <section class="mt-8">
      <h4>Syntactic Layer</h4>
      <ul>
        <li>SYN001: Edge targets exist.</li>
        <li>SYN002: Entry points reference existing nodes.</li>
        <li>SYN003: Predicate references exist.</li>
        <li>SYN004: No duplicate node IDs.</li>
      </ul>
      <h4>Semantic Layer</h4>
      <ul>
        <li>SEM001: Auth nodes have predicates.</li>
        <li>SEM002: External nodes have op codes.</li>
        <li>SEM003: Terminal nodes shouldn't have outgoing edges.</li>
        <li>SEM004: No cycles (must be a DAG).</li>
      </ul>
      <h4>Pragmatic Layer</h4>
      <ul>
        ${DATA.constraints_pragmatic.map(c => `<li><span class="badge">${c.code}</span> ${c.check}</li>`).join('')}
      </ul>
      <div class="mt-16">
        <h5>Constraint Decision Tree</h5>
        <div>
          <pre class="code-block"><code>Syntactic Valid?  ->  Semantic Valid?
      |                  |
      V                  V
  Pragmatic Valid?    Pass/Fail
  </code></pre>
        </div>
        <div style="font-size:13px;color:var(--color-text-secondary)">All compiled workflows must pass three layers to ensure safety and auditability.</div>
      </div>
    </section>`;
  }

  function renderBinaryRef() {
    return `<h2>Binary Format Reference</h2>
      <div class="mt-8">graph.bin byte layout:
      <pre class="hex-viewer"><code><span class="field">0x00</span> Magic (PXYZ)\n<span class="field">0x04</span> Version major\n<span class="field">0x06</span> Version minor\n<span class="field">0x08</span> Node count\n<span class="field">0x0C</span> Edge count\n<span class="field">0x10</span> Predicate count\n<span class="field">0x14</span> String pool size\n<span class="field">0x18</span> Entry count\n<span class="field">0x20</span> Source hash\n<span class="field">0x40</span> Nodes offset\n<span class="field">0x44</span> Edges offset\n<span class="field">0x48</span> Predicates offset\n<span class="field">0x4C</span> Strings offset\n<span class="field">0x50</span> Entries offset</code></pre>
      <div>Node Entry (16 bytes):
      <pre class="hex-viewer"><code><span class="field">0x00</span> Node ID\n<span class="field">0x04</span> Kind\n<span class="field">0x05</span> Flags\n<span class="field">0x06</span> Op code\n<span class="field">0x08</span> Data offset\n<span class="field">0x0C</span> Edge start index\n<span class="field">0x0E</span> Edge count</code></pre></div>
      <div>Edge Entry (12 bytes):
      <pre class="hex-viewer"><code><span class="field">0x00</span> Target node ID\n<span class="field">0x04</span> Predicate ID\n<span class="field">0x06</span> Reserved\n<span class="field">0x08</span> Weight\n<span class="field">0x0A</span> Flags</code></pre></div>
      <div class="mt-8">All multi-byte fields are little-endian. Unique strings stored in a deduplicated pool.</div>
    </div>`;
  }

  function renderExampleWorkflow() {
    // show a flowchart for the sample approval workflow
    return `<h2>Example Workflow</h2>
      <div class="mt-8"><strong>Approval Workflow XML</strong></div>
      <pre class="code-block" id="example-xml"><code>${syntaxHighlightXML(DATA.example_workflow.replace(/</g,'&lt;').replace(/>/g,'&gt;'))}</code></pre>
      <button class="copy-btn" onclick="copyWorkflowXML()">Copy XML</button>
      <div class="mt-16"><canvas id="flowchart-canvas" class="flowchart-canvas"></canvas></div>
      <div class="mt-8">This workflow: <ul>
        <li>Starts with user input <span class="badge">transform</span></li>
        <li>Auth check via predicate <span class="badge">auth</span></li>
        <li>If approved: triggers <span class="badge irrevocable">external EMAIL_SEND</span> op</li>
        <li>Success terminal or error on auth failure</li>
      </ul></div>
      <div class="mt-8">
        <strong>Conceptual compiled binary:</strong>
        <pre class="hex-viewer"><code>PXYZ | V1.0 | Nodes:5 | Edges:4 | Preds:1 | ...</code></pre>
        <span style="font-size:13px;color:var(--color-text-secondary);">Actual graph.bin encodes node structure, predicate bytecode, and string pool offsets, as defined above.</span>
      </div>
      <div class="mt-16">
        <label for="workflow-select">Show another workflow:</label>
        <select id="workflow-select" class="form-control" style="max-width:240px">
          <option value="approval">Approval Workflow</option>
          <option value="data-validation">Data Validation Workflow</option>
          <option value="third-party">Third-Party Integration</option>
          <option value="error-handling">Error Handling Workflow</option>
        </select>
      </div>
    `;
  }

  function initGlossarySidebar() {
    const sidebar = document.getElementById('glossary-sidebar');
    sidebar.innerHTML = '<h3 class="mb-8">Glossary</h3>' + GLOSSARY.map(e => '<div class="glossary-term">' + e.term + '</div><div class="glossary-desc mb-8">' + e.desc + '</div>').join('');
  }

  function copyWorkflowXML() {
    navigator.clipboard.writeText(DATA.example_workflow);
  }

  // Tab event handling
  document.querySelectorAll('.tab-btn').forEach(btn => {
    btn.addEventListener('click', () => setTab(btn.dataset.tab));
  });
  setTab('overview');
  initGlossarySidebar();

  // Expander for pipeline section
  document.getElementById('pipeline-expander').addEventListener('click', () => {
    document.getElementById('pipeline-section').classList.toggle('hidden');
    document.getElementById('pipeline-expander').classList.toggle('active');
  });

  // Search handling
  document.getElementById('searchbar').addEventListener('input', e => {
    searchQuery = e.target.value;
    showTabContent(currentTab);
  });

  // Opcode tab search
  document.addEventListener('input', function (e) {
    if (e.target.id === 'opcode-search') {
      searchQuery = e.target.value;
      showTabContent('opcodes');
    }
  });

  // Example workflow select
  document.addEventListener('change', function(e) {
    if (e.target.id === 'workflow-select') {
      let val = e.target.value;
      let xml, desc, nodes, edges;
      if (val === 'approval') {
        xml = DATA.example_workflow;
        desc = `<ul>
          <li>User input validated</li>
          <li>Auth predicate checks permission</li>
          <li>Email send only if approved (irreversible)</li>
        </ul>`;
        nodes = ['start','auth_check','send_email','success','error'];
        edges = [['start','auth_check'],['auth_check','send_email'],['auth_check','error'],['send_email','success']];
      }
      else if (val === 'data-validation') {
        xml = '<omar>\n  <workflow>\n    <entry p=\"validate\" x=\"input\" node=\"validate\"/>\n    <nodes>\n      <node id=\"validate\" kind=\"transform\" schema=\"InputSchema\"/>\n      <node id=\"check\" kind=\"auth\" predicate=\"is_valid\"/>\n      <node id=\"result\" kind=\"render\" template=\"result\"/>\n      <node id=\"error\" kind=\"error\" status=\"422\"/>\n    </nodes>\n    <edges>\n      <edge from=\"validate\" to=\"check\"/>\n      <edge from=\"check\" to=\"result\"/>\n      <edge from=\"check\" to=\"error\" weight=\"0\"/>\n    </edges>\n  </workflow>\n  <predicates>\n    <predicate id=\"is_valid\">\n      <and>\n        <gt left=\"$input.query\" right=\"0\"/>\n        <lt left=\"$input.query\" right=\"100\"/>\n      </and>\n    </predicate>\n  </predicates>\n</omar>';
        desc = `<ul><li>Input validated (schema)</li><li>Auth check: value &gt; 0 and &lt; 100</li><li>Render result or error</li></ul>`;
        nodes = ['validate','check','result','error'];
        edges = [['validate','check'],['check','result'],['check','error']];
      }
      else if (val === 'third-party') {
        xml = '<omar>\n  <workflow>\n    <entry p=\"search\" x=\"api\" node=\"start\"/>\n    <nodes>\n      <node id=\"start\" kind=\"external\" op=\"0x0320\"/>\n      <node id=\"process\" kind=\"transform\"/>\n      <node id=\"show\" kind=\"render\" template=\"display\"/>\n    </nodes>\n    <edges>\n      <edge from=\"start\" to=\"process\"/>\n      <edge from=\"process\" to=\"show\"/>\n    </edges>\n  </workflow>\n</omar>';
        desc = `<ul><li>Search external API</li><li>Transform results</li><li>Render/display results</li></ul>`;
        nodes = ['start','process','show'];
        edges = [['start','process'],['process','show']];
      }
      else if (val === 'error-handling') {
        xml = '<omar>\n  <workflow>\n    <entry p=\"risky\" x=\"op\" node=\"start\"/>\n    <nodes>\n      <node id=\"start\" kind=\"external\" op=\"0x0360\"/>\n      <node id=\"fallback\" kind=\"error\" status=\"401\"/>\n    </nodes>\n    <edges>\n      <edge from=\"start\" to=\"fallback\" weight=\"0\"/>\n    </edges>\n  </workflow>\n</omar>';
        desc = `<ul><li>Risky external webhook call</li><li>Fallback error node on failure</li></ul>`;
        nodes = ['start','fallback'];
        edges = [['start','fallback']];
      }
      // update xml
      document.getElementById('example-xml').innerHTML = `<code>${syntaxHighlightXML(xml.replace(/</g,'&lt;').replace(/>/g,'&gt;'))}</code>`;
      // update copy logic
      document.querySelector('#tab-content .copy-btn').onclick = function() { navigator.clipboard.writeText(xml); };
      // flowchart redraw
      drawFlowchart(nodes, edges);
    }
  });
  // draw initial flowchart
  setTimeout(() => drawFlowchart(['start','auth_check','send_email','success','error'], [['start','auth_check'],['auth_check','send_email'],['auth_check','error'],['send_email','success']]), 50);

  function drawFlowchart(nodes, edges) {
    const canvas = document.getElementById('flowchart-canvas');
    if (!canvas) return;
    const ctx = canvas.getContext('2d');
    canvas.width = canvas.offsetWidth;
    canvas.height = canvas.offsetHeight;
    ctx.clearRect(0,0,canvas.width,canvas.height);
    let N = nodes.length;
    let spacing = Math.min(canvas.width/(N+1), 190);
    let xbase = (canvas.width - spacing * (N-1))/2;
    let ybase = canvas.height/2;
    let pos = nodes.map((n,i) => [xbase+i*spacing, ybase]);
    // Draw edges
    ctx.strokeStyle = "#5D878F";
    ctx.lineWidth = 2;
    edges.forEach(([a,b]) => {
      let ai = nodes.indexOf(a); let bi = nodes.indexOf(b);
      if (ai<0||bi<0) return;
      ctx.beginPath();
      ctx.moveTo(pos[ai][0],pos[ai][1]);
      ctx.lineTo(pos[bi][0],pos[bi][1]);
      ctx.stroke();
      // arrowhead
      let angle = Math.atan2(pos[bi][1]-pos[ai][1], pos[bi][0]-pos[ai][0]);
      let len = 18;
      ctx.beginPath();
      ctx.moveTo(pos[bi][0],pos[bi][1]);
      ctx.lineTo(pos[bi][0]-len*Math.cos(angle-Math.PI/7),pos[bi][1]-len*Math.sin(angle-Math.PI/7));
      ctx.lineTo(pos[bi][0]-len*Math.cos(angle+Math.PI/7),pos[bi][1]-len*Math.sin(angle+Math.PI/7));
      ctx.closePath();
      ctx.fillStyle="#1FB8CD";
      ctx.fill();
    });
    // Nodes
    nodes.forEach((n,i) => {
      ctx.beginPath();
      ctx.arc(pos[i][0],pos[i][1],28,0,2*Math.PI);
      ctx.fillStyle = i===2 ? '#DB4545' : (i===4 ? '#FFC185' : '#ECEBD5');
      ctx.fill();
      ctx.strokeStyle = '#5D878F'; ctx.lineWidth = 3; ctx.stroke();
      ctx.fillStyle = '#13343B'; ctx.font='bold 15px Geist, Inter, sans-serif';
      ctx.textAlign = 'center';
      ctx.fillText(n, pos[i][0], pos[i][1]+5);
    });
  }
});
