import { useEffect, useMemo, useRef, useState } from 'react'
import styled from 'styled-components'
import YAPV from '@yapv/core'
import SVG from '@yapv/svg'
import { useRecoilValue } from 'recoil'

import { sequenceState } from '../../state/atoms'
import { useElementSize } from '../../hooks/useElementSize'

const getNucleotideColor = nucleotide => {
    switch (nucleotide.toUpperCase()) {
        case 'A': return 'hsl(0,75%,60%)';
        case 'T': return 'hsl(50,85%,60%)';
        case 'G': return 'hsl(100,50%,60%)';
        case 'C': return 'hsl(200,50%,60%)';
        default: return 'hsl(0,0%,75%)';
    }
}

const PlasmidViewer = ({ className, name = "Foo" }) => {
    const ref = useRef(null)
    const sequence = useRecoilValue(sequenceState)
    const [renderer, setRenderer] = useState(null)
    const [parentRef, size] = useElementSize()

    const markers = useMemo(() => {
        return (sequence ?? []).map((nucleotide, i) => {
            return {
                displayConfig: {
                    width: 10,
                    distance: 100,
                    style: `stroke: transparent; fill: ${getNucleotideColor(nucleotide)}; stroke-width: 1;`,
                    anchor: {
                        width: 20,
                        height: 20,
                    },
                },
                location: {
                    start: i + 1,
                    end: i + 2
                },
            }
        })
    }, [sequence])

    const sequenceConfig = useMemo(() => {
        const interval =
            sequence.length < 3 ? 1 :
            sequence.length < 10 ? 2 :
            sequence.length < 50 ? 2 :
            sequence.length < 100 ? 10 :
            sequence.length < 500 ? 25 :
            sequence.length < 1000 ? 50 :
            sequence.length < 2000 ? 100 :
            sequence.length < 5000 ? 200 : 500
        return {
            sequenceConfig: {
                length: sequence.length,
            },
            displayConfig: {
                width: size.width,
                height: size.width,
                // viewBox: {
                //     width: size.width,
                //     height: size.width,
                // },
            },
            labels: [
                {
                    text: name,
                    displayConfig: {
                        type: 'text',
                        style: 'text-anchor: middle; font: 16pt "Arial", sans-serif; fill: black;',
                        vOffset: 0,
                    },
                },
                {
                    text: `${sequence.length} bp`,
                    displayConfig: {
                        type: 'text',
                        style: 'text-anchor: middle; font: 12pt "Arial", sans-serif; fill: black;',
                        vOffset: 20,
                    },
                },
            ],
            tracks: [
                {
                    displayConfig: {
                        distance: 100,
                        width: 0,
                        style: 'stroke: transparent; fill: transparent;',
                    },
                    axes: [{
                        displayConfig: {
                          distance: 5,
                          width: 5,
                          style: 'fill: black;',
                          scales: [{
                            width: 10,
                            distance: 5,
                            interval,
                            style: 'stroke: black; stroke-width: 2;',
                            label: {
                                type: 'text',
                                style: 'text-anchor: start; font: 12pt sans-serif; fill: black;',
                                distance: 30,
                            }
                          }]
                        }
                    }],
                    markers,
                },
            ],
        }
    }, [sequence, markers, size?.width])

    useEffect(() => {
        if (ref.current === null) return
        const renderer = YAPV.create(ref.current)
        renderer.use(SVG.circular)
        setRenderer(renderer)
    }, [])

    const rerender = () => {
        if (sequence.length === 0) {
            return
        }
        renderer?.draw(sequenceConfig)
    }

    useEffect(() => {
        if (
            ref.current === null
            || size.width === 0
            || size.height === 0
        ) return
        rerender()
    }, [size, sequence, renderer])

    useEffect(() => {
        if (
            ref.current === null
            || size.width === 0
            || size.height === 0
        ) return
        ref.current.replaceChildren()
        const renderer = YAPV.create(ref.current)
        renderer.use(SVG.circular)
        setRenderer(renderer)
    }, [size])

    return (
        <div ref={parentRef} className={className}>
            {sequence.length > 0 && (
                <div className="inner" ref={ref} />
            )}
        </div>
    )
}

export default styled(PlasmidViewer)`
    display: flex;
    flex-direction: column;
    justify-content: flex-start;
    align-items: center;
    width: 100%;
    height: 100%;

    & .inner {
        width: 100%;
        height: 100%;
    }

    & .inner > div {
        display: flex;
        flex-direction: column;
        justify-content: flex-start;
        align-items: flex-start;
        width: 100%;
        height: 100%;

        & > svg {
            width: 100%;
            height: auto;
        }
    }
`
