const statusClass: Record<string, string> = {
  inbox: 'status-inbox',
  brewing: 'status-brewing',
  ready: 'status-ready',
  producing: 'status-producing',
  shipped: 'status-shipped',
  archived: 'status-archived',
  active: 'status-producing'
};

export function statusChipClass(status: string): string {
  return statusClass[status] ?? 'status-inbox';
}
