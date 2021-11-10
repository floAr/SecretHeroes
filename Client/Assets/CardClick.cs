using UnityEngine;
using UnityEngine.EventSystems;

public class CardClick : MonoBehaviour, IPointerClickHandler
{
    // Define the function signature used to invoke a specific event
    public delegate void OnTargetClickedEventHandler(CardClick target);

    // Define the event invoked when the target will be clicked
    // The event will warn the entities subscribing to this event that the target has been clicked
    public event OnTargetClickedEventHandler OnTargetClickedEvent;

    // Detect when the Event System of Unity has detected a click on the target
    public void OnPointerClick(PointerEventData eventData)
    {
        // Invoke the event
        if (OnTargetClickedEvent != null)
            OnTargetClickedEvent(this);
    }
}
