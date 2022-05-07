import { useState, useRef, useCallback } from 'react'
import styled from 'styled-components'

const MenuButton = ({className, title, children}) => {
    const ref = useRef(null)
    const [isOpen, setIsOpen] = useState(false)

    const handleClick = useCallback(e => {
        if (e.target.parentElement !== ref.current) return
        setIsOpen(!isOpen)
    }, [isOpen])

    return (
        <div ref={ref} className={className} onClick={handleClick}>
            {isOpen && (
                <>
                    <div className="clickCatcher" onClick={handleClick} />
                    <div className="menu">
                        <div className="menu__content">
                            {children}
                        </div>
                    </div>
                </>
            )}
            <div className="title">{title}</div>
        </div>
    )
}

export default styled(MenuButton)`
    display: flex;
    align-items: center;
    padding: 0 .25rem 0 .75rem;
    font-weight: 500;
    border-left: 1px solid hsl(0,0%,90%);
    height: 100%;

    & .title {
        cursor: pointer;
    }

    & .clickCatcher {
        position: absolute;
        top: 0;
        left: 0;
        width: 100vw;
        height: 100vh;
        z-index: 100;
    }

    & .menu {
        position: relative;

        &__content {
            display: flex;
            flex-flow: column nowrap;
            position: absolute;
            left: -.5rem;
            top: .75rem;
            z-index: 200;
            min-width: 200px;
            background: hsla(0,0%,100%,.75);
            padding: .5rem .25rem;
            border-radius: .25rem;
            border: 1px solid hsl(0,0%,75%);
            box-shadow: 0 .1rem .1rem hsla(0,0%,0%,.25), 0 0 .25rem hsla(0,0%,0%,.25), 0 0 1rem hsla(0,0%,0%,.1);
            backdrop-filter: blur(.25rem);
        }
    }
`
