class ColorUtil {

    /**
     * Get the nucleotide color as HSL
     *
     * @param {string} nucleotide
     * @return {string}
     */
    static getNucleotideColor(nucleotide) {
        if (!nucleotide) return 'transparent'
        switch (nucleotide.toUpperCase()) {
            case 'A': return 'hsl(0,75%,20%)';
            case 'T': return 'hsl(50,75%,20%)';
            case 'G': return 'hsl(100,75%,20%)';
            case 'C': return 'hsl(150,75%,20%)';
            default: return 'hsl(0,0%,25%)';
        }
    }

    /**
     * Get the peptide color as HSL
     *
     * @param {string} peptide
     * @return {string}
     */
    static getPeptideColor(peptide) {
        if (!peptide) return 'transparent'
        switch (peptide.toUpperCase()) {
            case 'F':
            case 'Y':
                return 'hsl(260,33%,60%)'
            case 'L':
            case 'I':
            case 'V':
                return 'hsl(124,33%,60%)'
            case 'H':
                return 'hsl(290,33%,60%)'
            case 'P':
                return 'hsl(333,25%,60%)'
            case 'M':
            case 'C':
                return 'hsl(55,33%,60%)'
            case 'S':
            case 'T':
                return 'hsl(36,33%,60%)'
            case 'D':
            case 'E':
                return 'hsl(0,33%,60%)'
            case 'K':
            case 'R':
                return 'hsl(225,33%,60%)'
            case 'Q':
            case 'N':
                return 'hsl(175,33%,60%)'
            case 'A':
                return 'hsl(0,0%,50%)'
            case 'G':
                return 'hsl(0,0%,75%)'
            case '*':
                return 'hsl(36,25%,75%)'
            default:
                return 'hsl(0,50%,50%)'
        }
    }
}

export default ColorUtil
