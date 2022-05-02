export class SequenceDataItemModel {
    constructor(item) {
        this.data = item
    }

    /**
     * @returns {string[]}
     */
    codonLetters() {
        return this.data.codon
    }

    /**
     * @returns {string[]}
     */
    anticodonLetters() {
        return this.data.anticodon
    }

    /**
     * @returns {string}
     */
    peptideLetter() {
        return this.data.peptide ?? ''
    }

    /**
     * @returns {number}
     */
    startIndex() {
        return this.data.start_index
    }
}

export default class SequenceDataModel {
    constructor(data) {
        this._data = data ?? {
            sequence: [],
            bpCount: 0,
            cursor: {
                position: 0,
                isAtEnd: true,
            }
        }
        this._items = this._data.sequence.map(item => new SequenceDataItemModel(item))
    }

    /**
     * @returns {number}
     */
    bpCount() {
        return this._data.bpCount
    }

    /**
     * @returns {number}
     */
    cursorPosition() {
        return this._data.cursor.position
    }

    /**
     * @returns {boolean}
     */
    isCursorAtEnd() {
        return this._data.cursor.isAtEnd
    }

    /**
     * @param {SequenceDataItemModel} item
     * @returns {boolean}
     */
    isItemSelected(item) {
        const cursorPos = this.cursorPosition()
        const startIndex = item.startIndex()
        return cursorPos >= startIndex && cursorPos < startIndex + item.codonLetters().length
    }

    /**
     * @returns {SequenceDataItemModel[]}
     */
    items() {
        return this._items
    }

    /**
     * @returns {string[]}
     */
    nucleotideSequence() {
        return this.items().flatMap(item => item.codonLetters())
    }
}
