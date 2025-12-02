pub const STYLES: &str = r#"
* {
    box-sizing: border-box;
    margin: 0;
    padding: 0;
}

html {
    overflow-x: hidden;
    width: 100%;
    height: 100%;
}

body {
    font-family: 'Open Sans', -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif;
    padding: 0;
    margin: 0;
    background-color: #ffffff;
    color: #1a1a1b;
    overflow-x: hidden;
    width: 100%;
    height: 100%;
}

.app {
    font-family: 'Open Sans', sans-serif;
    display: flex;
    flex-direction: column;
    align-items: center;
    font-weight: 500;
    font-size: 13px;
    min-height: 100vh;
    width: 100%;
    max-width: 100vw;
    overflow-x: hidden;
}

/* Header */
.header {
    font-weight: 700;
    letter-spacing: 2px;
    font-size: 32px;
    color: #1a1a1b;
    width: 100%;
    border-bottom: 1px solid #d3d6da;
    text-align: center;
    padding: 15px 20px;
    margin-bottom: 20px;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 12px;
}

.header__title {
    white-space: nowrap;
}

.header__controls {
    display: flex;
    align-items: center;
    gap: 12px;
    flex-wrap: wrap;
    justify-content: center;
}

/* Language Select */
.language-select {
    font-family: 'Open Sans', sans-serif;
    font-size: 14px;
    font-weight: 600;
    padding: 8px 12px;
    border: 2px solid #d3d6da;
    border-radius: 4px;
    background-color: #ffffff;
    color: #1a1a1b;
    cursor: pointer;
    transition: border-color 0.2s ease;
}

.language-select:hover {
    border-color: #878a8c;
}

.language-select:focus {
    outline: none;
    border-color: #6aaa64;
}

/* Message Banner */
.message-banner {
    width: 100%;
    max-width: 330px;
    text-align: center;
    padding: 12px;
    font-size: 13px;
    font-weight: 500;
    margin: 10px 0;
    border-radius: 4px;
}

.message-banner--info {
    background-color: #f0f8ff;
    color: #1a1a1b;
    border: 1px solid #d3d6da;
}

.message-banner--success {
    background-color: #e8f5e9;
    color: #1a1a1b;
    border: 1px solid #6aaa64;
}

.message-banner--error {
    background-color: #ffebee;
    color: #1a1a1b;
    border: 1px solid #d32f2f;
}

/* Content */
.content {
    display: flex;
    flex-direction: column;
    align-items: center;
    width: 100%;
    max-width: 100%;
    padding: 20px 10px;
}

.section {
    margin-bottom: 30px;
    width: 100%;
    max-width: 330px;
    padding: 0 10px;
}

.section__title {
    font-size: 13px;
    font-weight: 600;
    color: #1a1a1b;
    text-align: center;
    margin-bottom: 10px;
    text-transform: uppercase;
}

/* Tile/Letter */
.tile {
    font-weight: 700;
    font-size: 32px;
    color: #fff;
    flex: 1;
    aspect-ratio: 1;
    max-width: 62px;
    max-height: 62px;
    display: flex;
    align-items: center;
    justify-content: center;
    text-transform: uppercase;
    box-sizing: border-box;
    user-select: none;
    transition: transform 0.1s ease;
}

.tile--default {
    border: 2px solid #d3d6da;
    color: #1a1a1b;
    background-color: #ffffff;
    cursor: pointer;
}

.tile--absent {
    background-color: #787c7e;
    color: #fff;
    border: none;
}

.tile--misplaced {
    background-color: #c9b458;
    color: #fff;
    border: none;
}

.tile--correct {
    background-color: #6aaa64;
    color: #fff;
    border: none;
}

.tile--inactive {
    cursor: default;
}

.tile:active:not(.tile--inactive) {
    transform: scale(0.95);
}

/* Small tiles for history */
.tile--small {
    width: 40px;
    height: 40px;
    font-size: 18px;
}

/* Word Row */
.word-row {
    display: flex;
    gap: 5px;
    justify-content: center;
    margin-bottom: 8px;
    width: 100%;
    max-width: 330px;
}

/* Game container */
.game {
    display: flex;
    gap: 5px;
    flex-direction: column;
}

/* Buttons */
.button-group {
    display: flex;
    flex-direction: column;
    gap: 8px;
    width: 100%;
    max-width: 330px;
    margin-top: 10px;
    padding: 0 10px;
    box-sizing: border-box;
}

.button {
    font-family: 'Open Sans', sans-serif;
    font-weight: 600;
    font-size: 14px;
    text-transform: uppercase;
    background-color: #6aaa64;
    color: #fff;
    border: 0px;
    border-radius: 4px;
    padding: 12px 0;
    cursor: pointer;
    transition: background-color 0.2s ease, transform 0.1s ease;
}

.button:hover:not(:disabled) {
    background-color: #5a9558;
}

.button:active:not(:disabled) {
    transform: translateY(1px);
}

.button:disabled {
    background-color: #d3d6da;
    cursor: auto;
}

.button--primary {
    background-color: #6aaa64;
}

.button--secondary {
    background-color: #878a8c;
}

.button--secondary:hover:not(:disabled) {
    background-color: #6e7175;
}

.button--yellow {
    background-color: #c9b458;
}

.button--yellow:hover:not(:disabled) {
    background-color: #b59f3b;
}

.button--red {
    background-color: #d32f2f;
}

.button--red:hover:not(:disabled) {
    background-color: #b71c1c;
}

.button--small {
    padding: 8px 12px;
    font-size: 14px;
    text-transform: none;
    font-weight: 600;
    border: 2px solid transparent;
    min-width: 100px;
}

/* History */
.history {
    max-height: 280px;
    overflow-y: auto;
    padding: 8px 0;
}

.history::-webkit-scrollbar {
    width: 8px;
}

.history::-webkit-scrollbar-track {
    background: #f1f1f1;
    border-radius: 4px;
}

.history::-webkit-scrollbar-thumb {
    background: #d3d6da;
    border-radius: 4px;
}

.history::-webkit-scrollbar-thumb:hover {
    background: #878a8c;
}

.history__empty {
    text-align: center;
    color: #878a8c;
    font-size: 13px;
    padding: 32px 0;
    font-style: italic;
}

/* Game Board */
.game-board {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 5px;
    width: 100%;
}

/* Instructions */
.instructions {
    text-align: center;
    color: #1a1a1b;
    font-size: 13px;
    line-height: 1.6;
}

.instructions p {
    margin-bottom: 8px;
}

.instructions__hints {
    display: flex;
    align-items: center;
    justify-content: flex-start;
    gap: 8px;
    margin-bottom: 6px;
    max-width: 330px;
    margin-left: auto;
    margin-right: auto;
}

.instructions__hints .tile {
    flex-shrink: 0;
}

/* Mobile Responsiveness */
@media (max-width: 480px) {
    .header {
        font-size: 24px;
        letter-spacing: 1px;
        padding: 12px 15px;
        gap: 10px;
    }

    .header__controls {
        gap: 10px;
    }

    .button--small {
        padding: 8px 12px;
        font-size: 13px;
    }

    .language-select {
        font-size: 13px;
        padding: 8px 12px;
    }

    .content {
        padding: 15px 20px;
    }

    .tile {
        max-width: none;
        max-height: none;
        font-size: clamp(20px, 5vw, 26px);
    }

    .tile--small {
        width: 30px;
        height: 30px;
        font-size: 14px;
        flex: 0 0 auto;
    }

    .word-row {
        gap: 4px;
        padding: 0 20px;
    }

    .button {
        font-size: 13px;
        padding: 8px 12px;
    }

    .section {
        max-width: 100%;
        padding: 0 20px;
    }

    .button-group {
        max-width: 100%;
        padding: 0 20px;
    }

    .message-banner {
        max-width: 100%;
        margin: 10px 20px;
    }
}

@media (max-width: 430px) {
    .tile {
        font-size: clamp(18px, 4.5vw, 24px);
    }

    .tile--small {
        width: 28px;
        height: 28px;
        font-size: 13px;
    }

    .word-row {
        gap: 3px;
    }
}

@media (max-width: 390px) {
    .header {
        font-size: 22px;
    }

    .tile {
        font-size: clamp(16px, 4vw, 22px);
    }

    .tile--small {
        width: 26px;
        height: 26px;
        font-size: 12px;
    }
}

@media (max-width: 360px) {
    .tile {
        font-size: clamp(14px, 3.5vw, 20px);
    }

    .tile--small {
        width: 24px;
        height: 24px;
        font-size: 11px;
    }

    .word-row {
        gap: 2px;
    }
}

/* Not Found Page */
.not-found {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    flex: 1;
    width: 100%;
    padding: 40px 20px;
}

.not-found__content {
    text-align: center;
    max-width: 500px;
}

.not-found__title {
    font-size: 120px;
    font-weight: 700;
    color: #6aaa64;
    margin: 0;
    line-height: 1;
}

.not-found__subtitle {
    font-size: 32px;
    font-weight: 700;
    color: #1a1a1b;
    margin: 20px 0 10px 0;
    letter-spacing: 1px;
}

.not-found__message {
    font-size: 16px;
    color: #878a8c;
    margin: 0 0 40px 0;
}

.not-found__actions {
    display: flex;
    gap: 12px;
    justify-content: center;
    flex-wrap: wrap;
}

.not-found__actions .button {
    min-width: 140px;
}

@media (max-width: 480px) {
    .not-found__title {
        font-size: 80px;
    }

    .not-found__subtitle {
        font-size: 24px;
    }

    .not-found__message {
        font-size: 14px;
    }

    .not-found__actions {
        flex-direction: column;
        width: 100%;
        max-width: 280px;
    }

    .not-found__actions .button {
        width: 100%;
    }
}

/* Footer */
.footer {
    margin-top: auto;
    padding: 20px 10px 40px 10px;
    text-align: center;
    font-size: 11px;
    color: #878a8c;
    width: 100%;
}

.footer a {
    color: #878a8c;
    text-decoration: none;
    transition: color 0.2s ease;
}

.footer a:hover {
    color: #6aaa64;
    text-decoration: underline;
}

@media (max-width: 480px) {
    .footer {
        font-size: 10px;
        padding: 15px 10px 30px 10px;
    }
}
"#;
