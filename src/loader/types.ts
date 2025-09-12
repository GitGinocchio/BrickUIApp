export interface BrickState {
  enabled: boolean;
  props: Record<string, any>;
  errors: Array<BrickError>;
}

export interface BrickError {
  brick: {
    name: string;
    author: string;
  };
  message: string;
  type: 'error' | 'warn';
  stack?: string;
  file?: string;
  line?: number;
  column?: number;
  isAsync?: boolean;
  timestamp?: number;
  context?: any;
  cause?: any;
  severity?: 'low'|'medium'|'high';
}
