import React from 'react';
import { usePoolClipBoardListener } from '../hooks';

const PoolClipboardBool: React.FC = () => {
    const { value, changeValue } = usePoolClipBoardListener();

    return (
        <div style={{ display: 'flex', alignItems: 'center', gap: '10px' }}>
            <span>Clipboard Status:</span>
            <button
                onClick={() => changeValue(!value)}
                style={{
                    padding: '8px 16px',
                    backgroundColor: value ? '#4caf50' : '#f44336',
                    color: '#fff',
                    border: 'none',
                    borderRadius: '4px',
                    cursor: 'pointer'
                }}
            >
                {value ? 'True' : 'False'}
            </button>
        </div>
    );
};

export default PoolClipboardBool;