/**
 * OMAR TOOL COMPILER
 * 
 * Transforms JSON tool definitions into OMAR XML workflows
 * 
 * Usage:
 *   const compiler = new OmarToolCompiler();
 *   const result = compiler.compile('tools/alexHormozi.json');
 *   // → { xml: '...', metadata: {...} }
 */

interface ToolDefinition {
  id: string;
  name: string;
  title: string;
  description: string;
  consultingFramework?: string;
  personality?: {
    style: string;
    philosophy: string;
    expertise: string[];
  };
  decisionTree?: {
    root: {
      question: string;
      branches: Branch[];
    };
  };
  consultingPrompts?: Record<string, string>;
  metadata?: {
    category: string;
    difficulty: string;
    bestFor: string[];
    keyTakeaway: string;
  };
  operations?: Record<string, {
    name: string;
    route: string;
    description: string;
    input: any;
    output?: any;
  }>;
}

interface Branch {
  condition: string;
  question: string;
  recommendation: string;
  framework?: Record<string, string>;
  metrics?: string[];
  output: string;
}

export class OmarToolCompiler {
  
  /**
   * Main compilation entry point
   */
  compile(toolDef: ToolDefinition): { xml: string; metadata: any } {
    const xml = this.generateXML(toolDef);
    const metadata = this.generateMetadata(toolDef);
    
    return { xml, metadata };
  }
  
  /**
   * Generate complete XML workflow from tool definition
   */
  private generateXML(tool: ToolDefinition): string {
    const parts: string[] = [];
    
    // XML header
    parts.push('<?xml version="1.0" encoding="UTF-8"?>');
    parts.push('<omar>');
    parts.push('');
    
    // Tool metadata
    parts.push('  <!-- TOOL METADATA -->');
    parts.push(`  <tool id="${this.escape(tool.id)}">`);
    parts.push(`    <name>${this.escape(tool.name)}</name>`);
    parts.push(`    <title>${this.escape(tool.title)}</title>`);
    parts.push(`    <description>${this.escape(tool.description)}</description>`);
    
    if (tool.metadata) {
      parts.push(`    <category>${this.escape(tool.metadata.category)}</category>`);
      parts.push(`    <difficulty>${this.escape(tool.metadata.difficulty)}</difficulty>`);
    }
    
    if (tool.personality?.expertise) {
      parts.push('    <expertise>');
      tool.personality.expertise.forEach(exp => {
        parts.push(`      <item>${this.escape(exp)}</item>`);
      });
      parts.push('    </expertise>');
    }
    
    parts.push('  </tool>');
    parts.push('');
    
    // Schemas
    parts.push(this.generateSchemas(tool));
    parts.push('');
    
    // Predicates (from decision tree conditions)
    if (tool.decisionTree) {
      parts.push(this.generatePredicates(tool));
      parts.push('');
    }
    
    // Workflow
    parts.push(this.generateWorkflow(tool));
    parts.push('');
    
    // Templates
    if (tool.decisionTree) {
      parts.push(this.generateTemplates(tool));
      parts.push('');
    }
    
    // Closing tag
    parts.push('</omar>');
    
    return parts.join('\n');
  }
  
  /**
   * Generate input/output schemas
   */
  private generateSchemas(tool: ToolDefinition): string {
    const parts: string[] = [];
    parts.push('  <!-- SCHEMAS -->');
    parts.push('  <schemas>');
    
    // Input schema
    parts.push(`    <schema id="${tool.id}_input">`);
    
    if (tool.decisionTree) {
      // Generate enum from branch conditions
      const conditions = tool.decisionTree.root.branches.map(b => b.condition);
      parts.push('      <field name="condition" type="string" required="true">');
      parts.push('        <enum>');
      conditions.forEach(cond => {
        parts.push(`          <value>${this.escape(cond)}</value>`);
      });
      parts.push('        </enum>');
      parts.push('      </field>');
    }
    
    parts.push('      <field name="context" type="object" required="false">');
    parts.push('        <field name="query" type="string"/>');
    parts.push('        <field name="currentMetrics" type="object"/>');
    parts.push('      </field>');
    parts.push('    </schema>');
    parts.push('');
    
    // Output schema
    parts.push(`    <schema id="${tool.id}_output">`);
    parts.push('      <field name="recommendation" type="string" required="true"/>');
    parts.push('      <field name="framework" type="object" required="false"/>');
    parts.push('      <field name="metrics" type="array" required="false"/>');
    parts.push('      <field name="analysis" type="string" required="true"/>');
    parts.push('    </schema>');
    
    parts.push('  </schemas>');
    
    return parts.join('\n');
  }
  
  /**
   * Generate predicates from decision tree branches
   */
  private generatePredicates(tool: ToolDefinition): string {
    if (!tool.decisionTree) return '';
    
    const parts: string[] = [];
    parts.push('  <!-- PREDICATES -->');
    parts.push('  <predicates>');
    
    tool.decisionTree.root.branches.forEach(branch => {
      const predicateId = `is_${branch.condition}`;
      parts.push(`    <predicate id="${predicateId}">`);
      parts.push('      <or>');
      parts.push(`        <eq left="$input.condition" right="${this.escape(branch.condition)}"/>`);
      
      // Add keyword matching from question and condition
      const keywords = this.extractKeywords(branch.condition, branch.question);
      keywords.forEach(keyword => {
        parts.push(`        <contains left="$input.context.query" right="${this.escape(keyword)}"/>`);
      });
      
      parts.push('      </or>');
      parts.push('    </predicate>');
      parts.push('');
    });
    
    parts.push('  </predicates>');
    
    return parts.join('\n');
  }
  
  /**
   * Generate main workflow from decision tree
   */
  private generateWorkflow(tool: ToolDefinition): string {
    const parts: string[] = [];
    parts.push('  <!-- WORKFLOW -->');
    parts.push(`  <workflow id="${tool.id}_consult">`);
    parts.push(`    <entry p="tool" x="${tool.id}" node="validate_input"/>`);
    parts.push('');
    parts.push('    <nodes>');
    
    // Validation node
    parts.push('      <!-- Input validation -->');
    parts.push('      <node id="validate_input" kind="transform">');
    parts.push(`        <schema ref="${tool.id}_input"/>`);
    parts.push('      </node>');
    parts.push('');
    
    if (tool.decisionTree) {
      // Classification node
      parts.push('      <!-- Classify challenge -->');
      parts.push('      <node id="classify_challenge" kind="auth">');
      parts.push(`        <description>${this.escape(tool.decisionTree.root.question)}</description>`);
      parts.push('      </node>');
      parts.push('');
      
      // Branch nodes
      tool.decisionTree.root.branches.forEach(branch => {
        parts.push(`      <!-- Branch: ${branch.condition} -->`);
        parts.push(`      <node id="${branch.condition}_framework" kind="transform">`);
        parts.push('        <operation ref="0x3000"/> <!-- TOOL_EXECUTE_FRAMEWORK -->');
        parts.push(`        <recommendation>${this.escape(branch.recommendation)}</recommendation>`);
        
        if (branch.framework) {
          parts.push('        <framework>');
          Object.entries(branch.framework).forEach(([key, value]) => {
            const order = key.replace('step', '');
            parts.push(`          <step id="${order}" order="${order}">`);
            parts.push(`            ${this.escape(value)}`);
            parts.push('          </step>');
          });
          parts.push('        </framework>');
        }
        
        if (branch.metrics) {
          parts.push('        <metrics>');
          branch.metrics.forEach(metric => {
            parts.push(`          <metric>${this.escape(metric)}</metric>`);
          });
          parts.push('        </metrics>');
        }
        
        parts.push('      </node>');
        parts.push('');
        
        // Render node
        parts.push(`      <node id="render_${branch.condition}" kind="render">`);
        parts.push(`        <template ref="${branch.condition}_output"/>`);
        parts.push('      </node>');
        parts.push('');
      });
      
      // Fallback node
      parts.push('      <!-- Fallback -->')
;
      parts.push('      <node id="no_match_fallback" kind="render">');
      parts.push('        <template ref="general_consulting"/>');
      parts.push('      </node>');
      parts.push('');
    }
    
    // Terminal node
    parts.push('      <node id="done" kind="terminal"/>');
    parts.push('    </nodes>');
    parts.push('');
    
    // Edges
    parts.push('    <edges>');
    parts.push('      <edge from="validate_input" to="classify_challenge">');
    parts.push('        <when><always/></when>');
    parts.push('      </edge>');
    parts.push('');
    
    if (tool.decisionTree) {
      tool.decisionTree.root.branches.forEach(branch => {
        const predicateId = `is_${branch.condition}`;
        
        parts.push(`      <!-- Branch: ${branch.condition} -->`);
        parts.push(`      <edge from="classify_challenge" to="${branch.condition}_framework">`);
        parts.push(`        <when><predicate ref="${predicateId}"/></when>`);
        parts.push('      </edge>');
        parts.push(`      <edge from="${branch.condition}_framework" to="render_${branch.condition}">`);
        parts.push('        <when><always/></when>');
        parts.push('      </edge>');
        parts.push(`      <edge from="render_${branch.condition}" to="done">`);
        parts.push('        <when><always/></when>');
        parts.push('      </edge>');
        parts.push('');
      });
      
      // Fallback edge
      parts.push('      <!-- Fallback edge -->');
      parts.push('      <edge from="classify_challenge" to="no_match_fallback">');
      parts.push('        <when>');
      parts.push('          <and>');
      tool.decisionTree.root.branches.forEach(branch => {
        parts.push(`            <not><predicate ref="is_${branch.condition}"/></not>`);
      });
      parts.push('          </and>');
      parts.push('        </when>');
      parts.push('      </edge>');
      parts.push('      <edge from="no_match_fallback" to="done">');
      parts.push('        <when><always/></when>');
      parts.push('      </edge>');
    }
    
    parts.push('    </edges>');
    parts.push('  </workflow>');
    
    return parts.join('\n');
  }
  
  /**
   * Generate output templates
   */
  private generateTemplates(tool: ToolDefinition): string {
    if (!tool.decisionTree) return '';
    
    const parts: string[] = [];
    parts.push('  <!-- TEMPLATES -->');
    parts.push('  <templates>');
    
    tool.decisionTree.root.branches.forEach(branch => {
      parts.push(`    <template id="${branch.condition}_output">`);
      parts.push('      <![CDATA[');
      parts.push(`# ${this.capitalizeWords(branch.condition.replace(/_/g, ' '))}`);
      parts.push('');
      parts.push(`## ${tool.name}'s Recommendation`);
      parts.push('');
      parts.push('{{recommendation}}');
      parts.push('');
      
      if (branch.framework) {
        parts.push('## Implementation Framework');
        parts.push('');
        parts.push('{{#each framework.steps}}');
        parts.push('{{order}}. {{this}}');
        parts.push('{{/each}}');
        parts.push('');
      }
      
      if (branch.metrics) {
        parts.push('## Key Metrics');
        parts.push('');
        parts.push('{{#each metrics}}');
        parts.push('- {{this}}');
        parts.push('{{/each}}');
        parts.push('');
      }
      
      parts.push('---');
      parts.push('');
      parts.push(`*Consulting framework by ${tool.name}*`);
      parts.push('      ]]>');
      parts.push('    </template>');
      parts.push('');
    });
    
    parts.push('  </templates>');
    
    return parts.join('\n');
  }
  
  /**
   * Generate metadata for tool registry
   */
  private generateMetadata(tool: ToolDefinition): any {
    return {
      id: tool.id,
      name: tool.name,
      title: tool.title,
      description: tool.description,
      category: tool.metadata?.category || 'general',
      difficulty: tool.metadata?.difficulty || 'intermediate',
      expertise: tool.personality?.expertise || [],
      branches: tool.decisionTree?.root.branches.length || 0,
      operationCode: this.assignOperationCode(tool),
      graphFile: `${tool.id}.graph.bin`,
      xmlFile: `${tool.id}.xml`,
    };
  }
  
  /**
   * Assign operation code based on tool category
   */
  private assignOperationCode(tool: ToolDefinition): string {
    const category = tool.metadata?.category || 'general';
    
    // Business legends: 0x3010-0x302F (19 codes)
    if (category.includes('business') || category.includes('scaling')) {
      return '0x3010'; // Base code, actual assignment during compilation
    }
    
    // Tech legends: 0x3030-0x306F (36 codes)
    if (category.includes('computing') || category.includes('engineering')) {
      return '0x3030'; // Base code
    }
    
    // Domain tools: 0x3070-0x308F (18 codes)
    if (category.includes('analysis') || category.includes('tool')) {
      return '0x3070'; // Base code
    }
    
    // Vector ops: 0x3090-0x309F
    if (category.includes('vector') || category.includes('rag')) {
      return '0x3090';
    }
    
    // System services: 0x30A0-0x30AF
    if (category.includes('memory') || category.includes('service')) {
      return '0x30A0';
    }
    
    return '0x3000'; // Default
  }
  
  /**
   * Extract keywords from condition and question
   */
  private extractKeywords(condition: string, question: string): string[] {
    const keywords = new Set<string>();
    
    // From condition (e.g., "customer_acquisition" → ["customer", "acquisition"])
    condition.split('_').forEach(word => {
      if (word.length > 3) keywords.add(word.toLowerCase());
    });
    
    // From question (extract meaningful words)
    const words = question.toLowerCase().match(/\b\w{4,}\b/g) || [];
    words.forEach(word => {
      if (!['what', 'when', 'where', 'which', 'have', 'this', 'that', 'with'].includes(word)) {
        keywords.add(word);
      }
    });
    
    return Array.from(keywords);
  }
  
  /**
   * Escape XML special characters
   */
  private escape(str: string): string {
    return str
      .replace(/&/g, '&amp;')
      .replace(/</g, '&lt;')
      .replace(/>/g, '&gt;')
      .replace(/"/g, '&quot;')
      .replace(/'/g, '&apos;');
  }
  
  /**
   * Capitalize words for titles
   */
  private capitalizeWords(str: string): string {
    return str.split(' ')
      .map(word => word.charAt(0).toUpperCase() + word.slice(1).toLowerCase())
      .join(' ');
  }
}

// Example usage
if (require.main === module) {
  const fs = require('fs');
  const path = require('path');
  
  const compiler = new OmarToolCompiler();
  
  // Compile Alex Hormozi tool
  const toolPath = path.join(__dirname, '../alexHormozi.json');
  const toolDef = JSON.parse(fs.readFileSync(toolPath, 'utf-8'));
  
  const { xml, metadata } = compiler.compile(toolDef);
  
  // Write outputs
  const outputDir = path.join(__dirname, '../compiled_tools');
  if (!fs.existsSync(outputDir)) {
    fs.mkdirSync(outputDir, { recursive: true });
  }
  
  fs.writeFileSync(path.join(outputDir, `${toolDef.id}.xml`), xml);
  fs.writeFileSync(path.join(outputDir, `${toolDef.id}.meta.json`), JSON.stringify(metadata, null, 2));
  
  console.log(`✅ Compiled ${toolDef.name}`);
  console.log(`   XML: ${toolDef.id}.xml`);
  console.log(`   Metadata: ${toolDef.id}.meta.json`);
}

export default OmarToolCompiler;
