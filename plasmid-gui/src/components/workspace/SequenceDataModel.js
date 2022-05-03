export class SequenceDataSelectionModel {
    constructor(data) {
        this._data = data
    }

    /**
     * @returns {number|null}
     */
    get start() {
        return this._data?.start
    }

    /**
     * @returns {number|null}
     */
    get end() {
        return this._data?.end
    }

    /**
     * @returns {boolean}
     */
    get isActive() {
        return this._data !== null && this._data !== undefined
    }

    contains(index) {
        if (!this.isActive) return false
        return index >= this.start && index < this.end
    }
}

export class SequenceDataCursorModel {
    constructor(data) {
        this._data = data ?? { position: 0, is_at_end: true }
    }

    /**
     * @returns {number}
     */
     get cursorPosition() {
        return this._data.position
    }

    /**
     * @returns {boolean}
     */
    isCursorAtEnd() {
        return this._data.is_at_end
    }

    /**
     * @param {SequenceDataItemModel} item
     * @returns {boolean}
     */
     isItemSelected(item) {
        const cursorPos = this.cursorPosition
        const startIndex = item.startIndex
        return cursorPos >= startIndex && cursorPos < startIndex + item.codonLetters.length
    }
}

export class SequenceDataItemModel {
    constructor(item) {
        this.data = item
    }

    /**
     * @returns {string[]}
     */
    get codonLetters() {
        return this.data.codon
    }

    /**
     * @returns {string[]}
     */
    get anticodonLetters() {
        return this.data.anticodon
    }

    /**
     * @returns {string}
     */
    get peptideLetter() {
        return this.data.peptide ?? ''
    }

    /**
     * @returns {number}
     */
    get startIndex() {
        return this.data.start_index
    }
}

export default class SequenceDataModel {
    constructor(data) {
        const patchedData = {
            sequence: data?.sequence ?? [],
            bp_count: data?.bp_count ?? 0,
        }
        this._data = patchedData
        this._items = this._data.sequence.map(item => new SequenceDataItemModel(item))
        this._selection = new SequenceDataSelectionModel(this._data.selection)
    }

    /**
     * @returns {number}
     */
    get bpCount() {
        return this._data.bp_count
    }

    /**
     * @returns {SequenceDataItemModel[]}
     */
    get items() {
        return this._items
    }

    /**
     * @returns {string[]}
     */
    get nucleotideSequence() {
        return this.items.flatMap(item => item.codonLetters)
    }

    /**
     * @returns {string[]}
     */
     get antinucleotideSequence() {
        return this.items.flatMap(item => item.anticodonLetters)
    }
}
