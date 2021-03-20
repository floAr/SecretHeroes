using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class TransitionManager : MonoBehaviour
{

    public CameraController MainCam;
    public FadeCamera MainCamFade;

    public Transform ResetTransform;

    public Coroutine RunningTransition;

    public ClickableObject Market;

    public DrawManager DrawHall;

    [ContextMenu("Market Transition")]
    public void TransitionIntoMarket()
    {
        StartCoroutine(MarketTransition());
    }

    public IEnumerator MarketTransition()
    {
        MainCam.LerpToTransform(Market.ObjectCamera.transform.position, Market.ObjectCamera.transform.rotation.eulerAngles);
        yield return new WaitForSeconds(MainCam.LerpTime- MainCamFade.Duration * 0.95f);
        MainCamFade.FadeOut();
        yield return new WaitForSeconds(MainCamFade.Duration);
        MainCam.transform.position = DrawHall.DrawCamera.transform.position;
        MainCam.transform.rotation = DrawHall.DrawCamera.transform.rotation;
        MainCamFade.FadeIn();
        yield return true;
    }

    [ContextMenu("Reset")]
    public void ResetTransitions()
    {
        StartCoroutine(ResetTransition());
    }

    public IEnumerator ResetTransition()
    {
        MainCamFade.FadeOut();
        yield return new WaitForSeconds(MainCamFade.Duration);
        MainCam.transform.position = ResetTransform.position;
        MainCam.transform.rotation = ResetTransform.rotation;
        MainCamFade.FadeIn();
        yield return true;
    }
}
