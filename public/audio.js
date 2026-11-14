// Web Audio API Synthesizer for Conquián Mexicano
// Zero external audio files required!

class ConquianAudio {
  constructor() {
    this.ctx = null;
    this.enabled = true;
    this.speechEnabled = false;
    this.spanishVoice = null;
    this.englishVoice = null;

    if ('speechSynthesis' in window) {
      const loadVoices = () => {
        const voices = window.speechSynthesis.getVoices();
        this.spanishVoice =
          voices.find(v => v.lang.includes('MX') || v.lang.startsWith('es')) || null;
        this.englishVoice =
          voices.find(v => v.lang.startsWith('en')) || null;
      };
      loadVoices();
      window.speechSynthesis.onvoiceschanged = loadVoices;
    }
  }

  init() {
    if (!this.ctx) {
      const AudioContext = window.AudioContext || window.webkitAudioContext;
      this.ctx = new AudioContext();
    }
    if (this.ctx && this.ctx.state === 'suspended') {
      this.ctx.resume();
    }
  }

  playFlip() {
    if (!this.enabled) return;
    this.init();
    const t = this.ctx.currentTime;
    const osc = this.ctx.createOscillator();
    const gain = this.ctx.createGain();
    osc.type = 'triangle';
    osc.frequency.setValueAtTime(260, t);
    osc.frequency.exponentialRampToValueAtTime(520, t + 0.08);
    gain.gain.setValueAtTime(0.24, t);
    gain.gain.exponentialRampToValueAtTime(0.01, t + 0.09);
    osc.connect(gain);
    gain.connect(this.ctx.destination);
    osc.start(t);
    osc.stop(t + 0.09);
  }

  playDiscard() {
    if (!this.enabled) return;
    this.init();
    const t = this.ctx.currentTime;
    const osc = this.ctx.createOscillator();
    const gain = this.ctx.createGain();
    osc.type = 'sine';
    osc.frequency.setValueAtTime(220, t);
    osc.frequency.exponentialRampToValueAtTime(110, t + 0.09);
    gain.gain.setValueAtTime(0.22, t);
    gain.gain.exponentialRampToValueAtTime(0.01, t + 0.09);
    osc.connect(gain);
    gain.connect(this.ctx.destination);
    osc.start(t);
    osc.stop(t + 0.09);
  }

  playPass() {
    if (!this.enabled) return;
    this.init();
    const t = this.ctx.currentTime;
    const osc = this.ctx.createOscillator();
    const gain = this.ctx.createGain();
    osc.type = 'sine';
    osc.frequency.setValueAtTime(360, t);
    osc.frequency.exponentialRampToValueAtTime(280, t + 0.05);
    gain.gain.setValueAtTime(0.14, t);
    gain.gain.exponentialRampToValueAtTime(0.01, t + 0.05);
    osc.connect(gain);
    gain.connect(this.ctx.destination);
    osc.start(t);
    osc.stop(t + 0.05);
  }

  playMeld() {
    if (!this.enabled) return;
    this.init();
    const t = this.ctx.currentTime;
    const notes = [440, 554.37, 659.25]; // A major chord
    notes.forEach((freq, i) => {
      const osc = this.ctx.createOscillator();
      const gain = this.ctx.createGain();
      osc.type = 'triangle';
      osc.frequency.setValueAtTime(freq, t + i * 0.045);
      gain.gain.setValueAtTime(0.22, t + i * 0.045);
      gain.gain.exponentialRampToValueAtTime(0.005, t + i * 0.045 + 0.28);
      osc.connect(gain);
      gain.connect(this.ctx.destination);
      osc.start(t + i * 0.045);
      osc.stop(t + i * 0.045 + 0.28);
    });
  }

  playObligado() {
    if (!this.enabled) return;
    this.init();
    const t = this.ctx.currentTime;
    const notes = [330, 415.3, 622.25];
    notes.forEach((freq, i) => {
      const osc = this.ctx.createOscillator();
      const gain = this.ctx.createGain();
      osc.type = 'sawtooth';
      osc.frequency.setValueAtTime(freq, t + i * 0.05);
      gain.gain.setValueAtTime(0.16, t + i * 0.05);
      gain.gain.exponentialRampToValueAtTime(0.005, t + i * 0.05 + 0.35);
      osc.connect(gain);
      gain.connect(this.ctx.destination);
      osc.start(t + i * 0.05);
      osc.stop(t + i * 0.05 + 0.35);
    });
  }

  playTie() {
    if (!this.enabled) return;
    this.init();
    const t = this.ctx.currentTime;
    [220, 196].forEach((freq, i) => {
      const osc = this.ctx.createOscillator();
      const gain = this.ctx.createGain();
      osc.type = 'triangle';
      osc.frequency.setValueAtTime(freq, t + i * 0.22);
      gain.gain.setValueAtTime(0.25, t + i * 0.22);
      gain.gain.exponentialRampToValueAtTime(0.01, t + i * 0.22 + 0.24);
      osc.connect(gain);
      gain.connect(this.ctx.destination);
      osc.start(t + i * 0.22);
      osc.stop(t + i * 0.22 + 0.24);
    });
  }

  playYourTurn() {
    if (!this.enabled) return;
    this.init();
    const t = this.ctx.currentTime;
    const osc = this.ctx.createOscillator();
    const gain = this.ctx.createGain();
    osc.type = 'sine';
    osc.frequency.setValueAtTime(587.33, t);
    osc.frequency.setValueAtTime(783.99, t + 0.07);
    gain.gain.setValueAtTime(0.18, t);
    gain.gain.exponentialRampToValueAtTime(0.005, t + 0.25);
    osc.connect(gain);
    gain.connect(this.ctx.destination);
    osc.start(t);
    osc.stop(t + 0.25);
  }

  playFanfare() {
    if (!this.enabled) return;
    this.init();
    const t = this.ctx.currentTime;
    const fanfareNotes = [
      { f: 392.00, start: 0.00, dur: 0.12 },
      { f: 523.25, start: 0.14, dur: 0.12 },
      { f: 659.25, start: 0.28, dur: 0.14 },
      { f: 783.99, start: 0.44, dur: 0.35 },
      { f: 659.25, start: 0.82, dur: 0.14 },
      { f: 783.99, start: 0.98, dur: 0.60 }
    ];
    fanfareNotes.forEach(({ f, start, dur }) => {
      const osc = this.ctx.createOscillator();
      const gain = this.ctx.createGain();
      osc.type = 'triangle';
      osc.frequency.setValueAtTime(f, t + start);
      gain.gain.setValueAtTime(0.32, t + start);
      gain.gain.exponentialRampToValueAtTime(0.01, t + start + dur);
      osc.connect(gain);
      gain.connect(this.ctx.destination);
      osc.start(t + start);
      osc.stop(t + start + dur);
    });
  }

  speak(text, lang = 'es') {
    if (!this.speechEnabled || !('speechSynthesis' in window) || !text) return;
    window.speechSynthesis.cancel();
    const utterance = new SpeechSynthesisUtterance(text);
    if (lang === 'es' && this.spanishVoice) {
      utterance.voice = this.spanishVoice;
      utterance.lang = this.spanishVoice.lang;
    } else if (lang === 'en' && this.englishVoice) {
      utterance.voice = this.englishVoice;
      utterance.lang = this.englishVoice.lang;
    } else {
      utterance.lang = lang === 'es' ? 'es-MX' : 'en-US';
    }
    utterance.rate = 1.02;
    window.speechSynthesis.speak(utterance);
  }
}

window.conquianAudio = new ConquianAudio();
