import styled from 'styled-components'

const Button = ({ className, onClick, children }) => {
    return (
        <div className={className}>
            {children}
        </div>
    )
}

export default styled(Button)`
    display: flex;
    align-items: center;
    background: hsl(0,0%,90%);
    padding: .5rem 1rem;
    border-radius: .25rem;
    box-shadow: 0 .1rem .1rem hsla(0,0%,0%,0.25);
    transition: all .1s ease;
    max-height: calc(100% - 1rem);

    &:hover {
        background: hsl(0,0%,80%);
        box-shadow: 0 .2rem .25rem hsla(0,0%,0%,0.25);
        user-select: none;
        cursor: pointer;
    }

    &:focus, &:active {
        background: hsl(0,0%,70%);
        box-shadow: 0 .2rem .25rem hsla(0,0%,0%,0.25);
        user-select: none;
        cursor: pointer;
    }
`
