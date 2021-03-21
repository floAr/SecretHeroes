using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class TransitionManager : MonoBehaviour
{
    public enum Location
    {
        MAIN,
        MARKET,
        BAR
    }

    public CameraController MainCam;
    public FadeCamera MainCamFade;

    public Transform ResetTransform;

    public Coroutine RunningTransition;

    public ClickableObject Market;
    public DrawManager DrawHall;


    public ClickableObject Bar;
    public SelectionRooster Selection;

    public Location CurrentLocation = Location.MAIN;

    [ContextMenu("Market Transition")]
    public void TransitionIntoMarket()
    {
        if (CurrentLocation == Location.MARKET)
            return;
        StartCoroutine(MarketTransition());
    }

    public IEnumerator MarketTransition()
    {
        if (CurrentLocation != Location.MAIN)
            yield return StartCoroutine(ResetTransition());
        CurrentLocation = Location.MARKET;
        MainCam.LerpToTransform(Market.ObjectCamera.transform.position, Market.ObjectCamera.transform.rotation.eulerAngles);
        yield return new WaitForSeconds(MainCam.LerpTime - MainCamFade.Duration * 0.95f);
        MainCamFade.FadeOut();
        yield return new WaitForSeconds(MainCamFade.Duration);
        MainCam.transform.position = DrawHall.DrawCamera.transform.position;
        MainCam.transform.rotation = DrawHall.DrawCamera.transform.rotation;
        MainCamFade.FadeIn();
        yield return true;
    }


    [ContextMenu("Bar Transition")]
    public void TransitionIntoBar()
    {
        if (CurrentLocation == Location.BAR)
            return;
        StartCoroutine(BarTransition());
    }

    public IEnumerator BarTransition()
    {
        if (CurrentLocation != Location.MAIN)
            yield return StartCoroutine(ResetTransition());
        CurrentLocation = Location.BAR;
        MainCam.LerpToTransform(Bar.ObjectCamera.transform.position, Bar.ObjectCamera.transform.rotation.eulerAngles);
        yield return new WaitForSeconds(MainCam.LerpTime - MainCamFade.Duration * 0.95f);
        MainCamFade.FadeOut();
        yield return new WaitForSeconds(MainCamFade.Duration);
        MainCam.transform.position = Selection.SectionCamera.transform.position;
        MainCam.transform.rotation = Selection.SectionCamera.transform.rotation;
        MainCamFade.FadeIn();
        yield return true;
    }




    [ContextMenu("Reset")]
    public void ResetTransitions()
    {
        if (CurrentLocation == Location.MAIN)
            return;
        CurrentLocation = Location.MAIN;
        StartCoroutine(ResetTransition());
    }

    public IEnumerator ResetTransition()
    {
        MainCamFade.FadeOut();
        yield return new WaitForSeconds(MainCamFade.Duration);
        MainCam.transform.position = ResetTransform.position;
        MainCam.transform.rotation = ResetTransform.rotation;
        MainCamFade.FadeIn();
        CurrentLocation = Location.MAIN;
        yield return true;
    }
}
