/**
 * @license
 * SPDX-License-Identifier: Apache-2.0
 */

export interface Vulnerability {
  id: string;
  title: string;
  severity: 'CRITICAL' | 'HIGH' | 'MEDIUM' | 'LOW';
  vector: string;
  description: string;
  remediation?: string;
  timestamp: string;
  status: 'OPEN' | 'TESTING' | 'EXPLOITED' | 'MITIGATED';
}

export interface AuditLog {
  id: string;
  timestamp: string;
  agent: 'SENTINEL-ALPHA' | 'AUDIT-OMEGA' | 'SYSTEM';
  message: string;
  type: 'INFO' | 'WARN' | 'EXPLOIT' | 'RESULT';
  metadata?: Record<string, any>;
}

export interface ExploitScenario {
  id: string;
  name: string;
  steps: string[];
  vulnerabilityId: string;
  successProbability: number;
}
