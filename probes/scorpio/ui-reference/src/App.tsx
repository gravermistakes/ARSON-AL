/**
 * @license
 * SPDX-License-Identifier: Apache-2.0
 */

import React, { useState, useEffect } from 'react';
import { 
  Plus, 
  Play, 
  Database, 
  AlertTriangle, 
  Search,
  Scan,
  Workflow,
  Terminal,
  Activity
} from 'lucide-react';
import { motion } from 'motion/react';
import { GoogleGenAI, Type } from "@google/genai";
import { AetheriaLayout } from '@/src/components/AetheriaLayout';
import { NerveCenter } from '@/src/components/NerveCenter';
import { VulnerabilityMapper } from '@/src/components/VulnerabilityMapper';
import { Vulnerability, AuditLog } from '@/src/types';
import { formatTimestamp, generateId } from '@/src/lib/utils';
import { MODELS, SYSTEM_INSTRUCTIONS, ai } from '@/src/lib/gemini';

export default function App() {
  const [logs, setLogs] = useState<AuditLog[]>([]);
  const [vulnerabilities, setVulnerabilities] = useState<Vulnerability[]>([]);
  const [isScanning, setIsScanning] = useState(false);
  const [targetCode, setTargetCode] = useState('');

  const addLog = (message: string, agent: AuditLog['agent'] = 'SYSTEM', type: AuditLog['type'] = 'INFO', metadata?: any) => {
    setLogs(prev => [...prev, {
      id: generateId(),
      timestamp: formatTimestamp(),
      agent,
      message,
      type,
      metadata
    }]);
  };

  const runVulnerabilityScan = async () => {
    if (!targetCode && vulnerabilities.length > 0) return;
    
    setIsScanning(true);
    setVulnerabilities([]);
    addLog('INITIATING RECURSIVE ATTENTION CYCLE...', 'SENTINEL-ALPHA');
    
    try {
      // Step 1: Sentinel Alpha Analyzes Attack Surface
      const sentinelResponse = await ai.models.generateContent({
        model: MODELS.PRECISE,
        contents: [
          { text: `Analyze the following infrastructure/code for vulnerabilities: ${targetCode || 'Generic standard web server configuration with exposing port 3000, using older express middleware.'}` }
        ],
        config: {
          systemInstruction: SYSTEM_INSTRUCTIONS.SENTINEL,
          responseMimeType: 'application/json',
          responseSchema: {
            type: Type.ARRAY,
            items: {
              type: Type.OBJECT,
              properties: {
                title: { type: Type.STRING },
                severity: { type: Type.STRING, enum: ['CRITICAL', 'HIGH', 'MEDIUM', 'LOW'] },
                vector: { type: Type.STRING },
                description: { type: Type.STRING },
                remediation: { type: Type.STRING }
              },
              required: ['title', 'severity', 'vector', 'description']
            }
          }
        }
      });

      const detectedVulns: any[] = JSON.parse(sentinelResponse.text || "[]");
      const mappedVulns: Vulnerability[] = detectedVulns.map(v => ({
        ...v,
        id: generateId(),
        timestamp: formatTimestamp(),
        status: 'OPEN'
      }));

      setVulnerabilities(mappedVulns);
      addLog(`MAPPED ${mappedVulns.length} FRAGILITY POINTS`, 'SENTINEL-ALPHA', 'RESULT');

      // Step 2: Audit Omega Processes Findings
      addLog('SYNCING WITH AUDIT-OMEGA FOR BEHAVIORAL PROFILING', 'SYSTEM');
      
      const auditorResponse = await ai.models.generateContent({
        model: MODELS.FLASH,
        contents: [
          { text: `Generate a brief threat intelligence report for these findings: ${JSON.stringify(mappedVulns)}` }
        ],
        config: {
          systemInstruction: SYSTEM_INSTRUCTIONS.AUDITOR,
        }
      });

      addLog(auditorResponse.text || 'Audit sync completed.', 'AUDIT-OMEGA', 'INFO');

      // Add a simulated exploit sequence
      if (mappedVulns.length > 0) {
        const topVuln = mappedVulns[0];
        setTimeout(() => {
          addLog(`SEQUENCING EXPLOIT FOR ${topVuln.title}`, 'SENTINEL-ALPHA', 'EXPLOIT');
          setTimeout(() => {
            setVulnerabilities(prev => prev.map(v => v.id === topVuln.id ? {...v, status: 'TESTING'} : v));
            addLog(`INJECTING SHELLCODE_FRAGMENT::${generateId().toUpperCase()}`, 'SENTINEL-ALPHA');
            setTimeout(() => {
               setVulnerabilities(prev => prev.map(v => v.id === topVuln.id ? {...v, status: 'EXPLOITED'} : v));
               addLog(`EXPLOIT SUCCESSFUL. ACCESS LEVEL: ROOT`, 'SENTINEL-ALPHA', 'RESULT', { target: topVuln.id });
            }, 3000);
          }, 2000);
        }, 1500);
      }

    } catch (error) {
      console.error(error);
      addLog(`FATAL::NEURAL_LINK_ERROR: ${error instanceof Error ? error.message : 'Unknown failure'}`, 'SYSTEM', 'WARN');
    } finally {
      setIsScanning(false);
    }
  };

  useEffect(() => {
    // Initial welcome message
    const timer = setTimeout(() => {
      addLog('AETHERIA SENTINEL ALPHA ONLINE', 'SENTINEL-ALPHA', 'INFO');
      addLog('AUDIT OMEGA MONITORING SUBSYSTEM ACTIVE', 'AUDIT-OMEGA', 'INFO');
      addLog('READY FOR ADVERSARIAL EXPLORATION', 'SYSTEM');
    }, 500);
    return () => clearTimeout(timer);
  }, []);

  return (
    <AetheriaLayout>
      <div className="p-6 h-full flex flex-col space-y-6">
        
        {/* Workspace Management */}
        <div className="flex flex-col md:flex-row md:items-end justify-between gap-6 pb-6 border-b border-edge">
          <div className="space-y-1">
            <div className="flex items-center space-x-2">
              <Scan className="w-4 h-4 text-accent" />
              <h2 className="text-sm font-mono font-bold text-white uppercase tracking-[0.4em]">Adversarial Explorer</h2>
            </div>
            <p className="text-zinc-600 text-[10px] font-mono tracking-widest uppercase">Target: LIVE_INFRA_INSTANCE_02 // neural scan v9.1</p>
          </div>
          <div className="flex items-center space-x-2">
             <div className="relative group">
                <button 
                  onClick={runVulnerabilityScan}
                  disabled={isScanning}
                  className={cn(
                    "relative flex items-center space-x-3 px-8 py-2.5 transition-all font-mono font-bold text-[10px] uppercase tracking-[0.2em] border",
                    isScanning 
                      ? "bg-zinc-900 text-zinc-600 border-edge cursor-not-allowed" 
                      : "bg-accent text-black border-accent hover:bg-white hover:border-white shadow-[0_0_15px_rgba(0,240,255,0.2)]"
                  )}
                >
                  {isScanning ? (
                    <>
                      <div className="w-3 h-3 border border-zinc-700 border-t-zinc-400 rounded-full animate-spin" />
                      <span>Processing...</span>
                    </>
                  ) : (
                    <>
                      <Play className="w-4 h-4 fill-current" />
                      <span>Initial Scan</span>
                    </>
                  )}
                </button>
             </div>
             <button className="flex items-center space-x-2 bg-zinc-900/50 text-zinc-500 border border-edge px-4 py-2.5 font-mono text-[10px] uppercase tracking-widest hover:text-zinc-300 transition-colors">
                <Workflow className="w-4 h-4" />
                <span>Strategy</span>
             </button>
          </div>
        </div>

        {/* Intelligence Split */}
        <div className="flex-1 grid grid-cols-1 lg:grid-cols-12 gap-6 min-h-0">
           {/* Scan Input & Mapped View */}
           <div className="lg:col-span-8 flex flex-col space-y-6 min-h-0">
              <div className="bg-void border border-edge relative flex flex-col h-48 shrink-0 group">
                <div className="bg-panel px-4 py-2 border-b border-edge flex items-center justify-between">
                   <div className="flex items-center space-x-2">
                     <Terminal className="w-3 h-3 text-accent" />
                     <span className="text-[9px] font-mono font-bold text-zinc-400 uppercase tracking-widest">Neural Input Surface</span>
                   </div>
                   <div className="text-[9px] font-mono text-zinc-700 uppercase">Awaiting instruction...</div>
                </div>
                <textarea
                  value={targetCode}
                  onChange={(e) => setTargetCode(e.target.value)}
                  placeholder="Paste adversarial context for autonomous exploration..."
                  className="flex-1 bg-transparent p-6 text-accent/80 font-mono text-[11px] resize-none focus:outline-none placeholder:text-zinc-800"
                />
                <div className="absolute bottom-4 right-4 pointer-events-none opacity-20">
                   <div className="text-[8px] font-mono text-accent uppercase tracking-[0.5em]">Target::Active_Pipe</div>
                </div>
              </div>

              <div className="flex-1 flex flex-col space-y-4 min-h-0">
                 <div className="flex items-center justify-between border-b border-edge/30 pb-2">
                    <div className="flex items-center space-x-2">
                       <Search className="w-3 h-3 text-accent opacity-50" />
                       <h3 className="text-[10px] font-mono font-bold text-zinc-400 uppercase tracking-widest">Fragility Analysis Map</h3>
                    </div>
                    <div className="text-[9px] font-mono text-zinc-600 uppercase">PROBABILITY_THRESHOLD: &gt; 85%</div>
                 </div>
                 <div className="flex-1 overflow-y-auto pr-2 custom-scrollbar">
                   <VulnerabilityMapper 
                      vulnerabilities={vulnerabilities} 
                      onSelect={(v) => addLog(`RECURSIVE_EXPANSION_INITIATED::${v.id}`, 'SENTINEL-ALPHA')} 
                    />
                 </div>
              </div>
           </div>
           
           {/* Thought Stream & Reports */}
           <div className="lg:col-span-4 flex flex-col space-y-6 min-h-0">
              <div className="flex flex-col flex-1 min-h-0 overflow-hidden">
                <NerveCenter logs={logs} />
              </div>

              <div className="bg-panel border border-edge p-5 shrink-0 relative overflow-hidden group">
                <div className="absolute top-0 right-0 p-2 opacity-50 group-hover:opacity-100 transition-opacity">
                   <div className="text-[8px] text-red-500 bg-red-950/20 px-2 py-0.5 border border-red-900 font-mono">ADAPTIVE STRATEGY</div>
                </div>
                <h3 className="text-[10px] font-mono font-bold text-white mb-3 uppercase tracking-widest border-b border-edge/30 pb-2">Automated Report_Summary</h3>
                <p className="text-[10px] text-zinc-500 leading-tight font-mono mb-4">
                   System mapping success: Memory sandbox breach probability high. Escalation path verified via Audit Omega recursive analysis.
                </p>
                <button className="w-full py-2.5 bg-zinc-900 border border-edge hover:bg-accent hover:text-black transition-all text-[10px] font-mono font-bold uppercase tracking-[0.3em]">
                   Export Intelligence
                </button>
              </div>
           </div>
        </div>

      </div>
    </AetheriaLayout>
  );
}
